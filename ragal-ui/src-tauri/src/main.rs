// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::{Path, PathBuf};
use std::process;
use std::io::prelude::*;
use std::fs;
use std::io;
use tauri::regex;
use serde::Deserialize;
use serde_yaml as yaml;

const TEMPLATE_BASE : &str = 
"base:
  title: ragal
  title_page: $TitlePage.html
  theme: $theme.css
  icon: $icon.ico
  mainresolution: [1080, 608]
";

const TEMPLATE_SRC: &str =
"src:
  entry: src/entry.md
  img: img
  standing: standing
  audio: audio
  custom_css: []
";


#[derive(Deserialize)]
struct ConfigSrc{
    entry: PathBuf,
    img: PathBuf,
    standing: PathBuf,
    audio: Option<PathBuf>,
    custom_css: Vec<PathBuf>,
    custom_script: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct ConfigBase{
    pub title: String,
    pub title_page: Option<PathBuf>,
    pub theme: Option<PathBuf>,
    pub icon: Option<PathBuf>,
    pub mainresolution: [u16; 2]
}

impl ConfigSrc {
    #[inline]
    fn is_all_relative(&self) -> bool{
        if self.entry.is_absolute()
        || self.img.is_absolute()
        || self.standing.is_absolute()
        || self.audio.as_ref().is_some_and(|t| t.is_absolute())
        || self.custom_script.as_ref().is_some_and(|t| t.is_absolute()){
            return false;
        }
        for ref t in self.custom_css.iter(){
            if t.is_absolute() {
                return false;
            }
        }
        true
    }
    fn to_stream_by(&self, base_space: &Path) -> Vec<PathBuf>{
        let mut res = Vec::with_capacity(7);
        res.push(base_space.join("init.js"));
        // res.push(base_space.join(self.entry));
        res.push(base_space.join(&self.img));
        res.push(base_space.join(&self.standing));
        if let Some(ref t) = self.audio{
            res.push(base_space.join(t));
        }
        if let Some(ref t) = self.custom_script {
            res.push(base_space.join(t));
        }
        for t in self.custom_css.iter(){
            res.push(base_space.join(t));
        }
        res
    } 
}

mod project {

    use super::*;
    use std::{collections::{BTreeMap, VecDeque}, ffi::OsStr, hash::{Hash, Hasher}};
    #[inline]
    fn open_file_or_dir(path: &Path) -> (&Path, &OsStr){
        let mut cfg_name = OsStr::new("ragal.cfg.yml");
        let dir_path = if path.is_file() {
            cfg_name = unsafe { path.file_name().unwrap_unchecked() };
            path.parent().unwrap_or(Path::new(""))
        }else{
            path
        };
        (dir_path,cfg_name)
    }

    pub(crate) fn new(path: &Path) -> tauri::Result<()> {
        let (dir_path, cfg_name) = open_file_or_dir(path);

        let mut cfg_f = fs::File::create(dir_path.join(cfg_name))?;
        cfg_f.write_all(TEMPLATE_BASE.as_bytes())?;
        cfg_f.write_all(TEMPLATE_SRC.as_bytes())?;
        cfg_f.write(b"game:\n")?;
        let src_path = dir_path.join("src");
        
        fs::create_dir(&src_path)?;
        fs::create_dir(dir_path.join("img"))?;
        fs::create_dir(dir_path.join("standing"))?;

        fs::File::create(src_path.join("entry.md"))?;
        fs::File::create(dir_path.join("init.js"))?;
        Ok(())
    }
    #[inline]
    fn in_base(base: &Path, path: &Path) -> bool {
        let t = path.to_string_lossy();
        let path = Path::new(t.trim_start_matches(r"\\?\"));
        path.starts_with(base)
    }


    pub(crate) fn bundle(path: &Path) -> Result<(), String>{
        use fs_extra::dir;
        use std::hash::DefaultHasher;

        let (dev_path, cfg_file) = open_file_or_dir(path);
        let dev_cfg_filep = dev_path.join(cfg_file);
        let dist_path = dev_path.join("dist");
        let reg = regex::Regex::new(r"\[(?<txt>.*?)\]\((?<url>.+?)\)").unwrap();

        let err2str = |err: io::Error| err.to_string();
        let tp = fs::read(&dev_cfg_filep).map_err(err2str)?;
        let Ok(cfg) = yaml::from_slice::<yaml::Mapping>(&tp)else{return Err("配置文件格式不符".into())};
        let Some(src) = cfg.get("src")else{return Err("无src键".into())};
        let Ok(src) = yaml::from_value::<ConfigSrc>(src.clone())else{return Err("资源路径配置不符".into())};
        let Some(base) = cfg.get("base")else{return Err("无base键".into())};
        let Ok(base) = yaml::from_value::<ConfigBase>(base.clone())else{return Err("资源路径配置不符".into())};
        if !src.is_all_relative(){
            return Err("工程资源路径都需为相对路径".into());
        }
        let mut md_map = BTreeMap::<PathBuf, PathBuf>::new();
        let mut bufp = VecDeque::with_capacity(20);
        let entry = dev_path.join(&src.entry);
        let jb_dir = entry.parent().unwrap();
        
        for e in walkdir::WalkDir::new(jb_dir)
            .into_iter()
        {
            let e = e.map_err(|err| err.to_string())?;
            if e.path().is_dir(){
                continue;
            }
            if !e.path().extension().is_some_and(|extension|extension == "md"){
                continue;
            }
            
            let project_p = e.path().strip_prefix(dev_path).map_err(|err| err.to_string()).unwrap();
            bufp.push_back(e.path().to_path_buf());
            md_map.insert(e.path().to_path_buf(), dist_path.join(project_p));
        }
        
        
        while let Some(txtp) = bufp.pop_front(){
            let txt = fs::read_to_string(&txtp).map_err(err2str)?;
            let modify_txt = reg.replace_all(txt.as_str(), |c: &regex::Captures|{
                let url = &c["url"];
                let url_p = Path::new(url);
                let url_p = if url_p.is_relative() {
                    let Ok(res) = txtp.parent().unwrap_or(Path::new("")).join(url_p).canonicalize()else{return c[0].to_string();};
                    let t = res.to_string_lossy();
                    let res = Path::new(t.trim_start_matches(r"\\?\"));
                    res.to_path_buf()
                }else{
                    url_p.to_path_buf()
                };
                if url_p.is_file() && url_p.extension().is_some_and(|val| val =="md"){
                    let re_urlp = if !in_base(dev_path, &url_p){
                        let file_name = url_p.file_stem().unwrap();
                        let mut dh = DefaultHasher::new();
                        url_p.hash(&mut dh);
                        let _code = dh.finish();
                        let code = unsafe {
                            let _b = &_code as *const u64 as *const u8;
                            std::slice::from_raw_parts(_b, 8)
                        };
                        format!("_glob/{}-{}", file_name.to_string_lossy(), ragal_b64::encode(&code))
                    }else{
                        let Ok(res) = url_p.strip_prefix(dev_path) else{return c[0].to_string();};
                        res.display().to_string()
                    };
                    if !md_map.contains_key(&url_p){
                        bufp.push_back(url_p.to_path_buf());
                        md_map.insert(url_p.clone(), dist_path.join(&re_urlp));
                    }
                    
                    return format!("[{}]($PROJECTROOT/{})", &c["txt"], &re_urlp);
                }
                
                c[0].to_string()
            });
            let Some(p) = md_map.get(&txtp)else{continue;};
            fs_extra::dir::create_all(p.parent().unwrap(), false).map_err(|err| err.to_string())?;
            let mut md = fs::File::create(p).map_err(err2str)?;
            md.write_all(modify_txt.as_bytes()).map_err(err2str)?;
        }
        let mut starter = fs::File::create_new(dist_path.join(base.title + ".exe")).map_err(err2str)?;
        starter.write_all(include_bytes!("../../ragal-linker/target/release/starter.exe")).map_err(err2str)?;
        fs::create_dir(dist_path.join("ragalRT")).map_err(err2str)?;
        fs::copy("ragalRT/ragalgame_exec.dll", dist_path.join("ragalRT/ragalgame_exec.dll")).map_err(err2str)?;
        fs_extra::copy_items(&src.to_stream_by(&dev_path), &dist_path, &dir::CopyOptions::default())
            .map_err(|e|e.to_string())?;
        fs::copy(dev_cfg_filep, dist_path.join("ragal.cfg.yml")).map_err(err2str)?;
        Ok(())
    }
} 

#[cfg(test)]
#[test]
fn check(){
    for e in walkdir::WalkDir::new("../lib-rs"){
        let e = e.unwrap();

        println!("{}", e.path().is_dir())
    }
}

#[tauri::command]
async fn new_project(path: PathBuf) -> Result<(), String> {
    project::new(&path).map_err(|err|err.to_string())
}

#[tauri::command]
async fn open_project(path: PathBuf) -> tauri::Result<()>{
    let Err(e) = process::Command::new("ragalRT/ragal-linker.exe")
        .arg("--run")
        .arg(path)
        .arg("--enable-devtool")
        .spawn()else{return Ok(())};
    Err(match e.kind() {
        io::ErrorKind::NotFound => io::Error::new(e.kind(), "ragal-launcher not found"),
        _ => e
    }.into())
}

#[tauri::command]
async fn bundle_project(path: PathBuf) -> Result<(), String>{
    project::bundle(&path)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_project, new_project, bundle_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
