use serde_yaml::from_str as str2yml;
use serde::{Serialize, Deserialize};
use std::path::{PathBuf, Path};

use tauri::Manager;

static TEMPLATE_BASE : &str = 
"title: ragal
title_page: $TitlePage.html
theme: $theme.css
icon: $icon.ico
mainresolution: [1080, 608]";

// #[allow(unused)]
// static TEMPLATE_SRC: &str =
// "entry: src/entry.md
// img: img
// standing: standing
// audio: audio
// custom_script: custom_script
// custom_css: []";
static TEMPLATE_GAME: &str =
"typing:
  speed: 4
  size: 2.9
standing: 
  1:
    - [30%, -10%, 100%]
  2: 
    - [10%, -10%, 100%]
    - [50%, -10%, 100%]
volume:
  bgm: 100
  se: 100";

trait ToResolve {
    fn to_resolve(&mut self, dir: impl AsRef<Path>);
}

pub trait StartBy<R: tauri::Runtime> : Manager<R> {
    fn start_by(&self, config: ProjConf) -> tauri::Result<()>;
}

impl<R: tauri::Runtime> StartBy<R> for tauri::AppHandle
where tauri::AppHandle: Manager<R>{

    fn start_by(&self, config: ProjConf) -> tauri::Result<()> {
        // use tauri::Icon;
        let wd = tauri::WindowBuilder::<R>::new(
        self, 
        "main", 
        tauri::WindowUrl::App(
                PathBuf::from("index.html")
            )
        )
        // .icon(Icon::File(config.base.icon))?
        .title(&config.base.title)
        .inner_size(config.base.mainresolution[0].into(), config.base.mainresolution[1].into())
        .build()?;
        wd.manage(config.src);
        wd.manage(config.game);
        wd.manage(config.base);
        Ok(())
    }
}

impl ToResolve for PathBuf {
    fn to_resolve(&mut self, dir: impl AsRef<Path>){
        if self.is_absolute(){
            return;
        }
        let new_path = dir.as_ref().join(&self);
        self.clone_from(&new_path);
    }
}

impl ToResolve for Option<PathBuf> {
    fn to_resolve(&mut self, dir: impl AsRef<Path>) {
        match self {
            Some(p) if p.is_relative() =>{
                p.clone_from(&dir.as_ref().join(&p))
            } 
            _ => ()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjConf{
    base: BaseConf,
    src: PathConf,
    game: GameConf
}

impl ProjConf {
    pub fn open_or_default(path: impl AsRef<Path>) -> Result<Self, String>{
        use std::fs::read_to_string;
        let path = path.as_ref();
        let mut this = match read_to_string(path){
            Ok(ref s) => {
                str2yml::<Self>(s).map_err(|err| err.to_string())?
            }
            // Err(err) if err.kind() == ErrorKind::NotFound => {
            //   str2yml::<Self>(TEMPLATE).unwrap()
            // },
            Err(err) => return Err(err.to_string())

        };
        // let reference = str2yml::<Self>(TEMPLATE).unwrap();

        let template_base = str2yml::<BaseConf>(TEMPLATE_BASE)
            .map_err(|err|err.to_string())?;
        let template_game = str2yml::<GameConf>(TEMPLATE_GAME)
            .map_err(|err|err.to_string())?;

        this.base.icon = this.base.icon.or(template_base.icon);
        this.base.theme = this.base.theme.or(template_base.theme);
        this.base.title_page = this.base.title_page.or(template_base.title_page);

        this.game.typing = this.game.typing.or(template_game.typing);
        this.game.standing = this.game.standing.or(template_game.standing);
        this.game.volume = this.game.volume.or(template_game.volume);

        let Some(path) = path.parent()else{return Err("配置文件路径有误".into())};
        this.base.icon.to_resolve(path);
        this.base.theme.to_resolve(path);
        this.base.title_page.to_resolve(path);
        this.src.entry.to_resolve(path);
        this.src.img.to_resolve(path);
        this.src.audio.to_resolve(path);
        this.src.standing.to_resolve(path);
        this.src.custom_script.to_resolve(path);
        Ok(this)
    }

    // pub fn base_config(&self) -> &BaseConf{
    //     &self.base
    // }
    // pub fn src_config(&self) -> &PathConf{
    //     &self.src
    // }
    // pub fn game_config(&self) -> &GameConf{
    //     &self.game
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseConf{
    pub title: String,
    pub title_page: Option<PathBuf>,
    pub theme: Option<PathBuf>,
    pub icon: Option<PathBuf>,
    pub mainresolution: [u16; 2]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathConf{
    pub entry: PathBuf,
    pub img: PathBuf,
    pub standing: PathBuf,
    pub audio: Option<PathBuf>,
    pub custom_script: Option<PathBuf>,
    pub custom_css: Vec<PathBuf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameConf{
    pub typing: Option<serde_yaml::Mapping>,
    pub standing: Option<serde_yaml::Mapping>,
    pub volume: Option<serde_yaml::Mapping>
}

// #[tauri::command]
// pub async fn open_project(app: tauri::AppHandle, config : PathBuf) -> tauri::Result<()>{
//     use std::io;
//     if !(config.exists() && config.is_file()){
//         return Err(tauri::Error::Io(io::Error::new(io::ErrorKind::NotFound, "NOT FOUND or NOT FILE")));
//     }
//     let config = ProjConf::open_or_default(&config).map_err(|err| tauri::Error::AssetNotFound(err))?;
//     app.start_by(config)
// }

#[tauri::command]
pub fn current_cfg_dir(state: tauri::State<'_, PathBuf>) -> PathBuf {
    state.to_path_buf()
}

#[tauri::command]
pub async fn game_config<R: tauri::Runtime>(window: tauri::Window<R>) -> serde_yaml::Value {
    let gameconf = window.state::<GameConf>().inner();
    serde_yaml::to_value(gameconf).unwrap()
}

#[tauri::command]
pub async fn src_config<R: tauri::Runtime>(window: tauri::Window<R>) -> serde_yaml::Value {
    let srcconf = window.state::<PathConf>().inner();
    serde_yaml::to_value(srcconf).unwrap()
}

#[tauri::command]
pub async fn custom_html<R: tauri::Runtime>(window: tauri::Window<R>, kind: String) -> Option<String> {
    let res= match kind.as_str() {
        "titlePage" => window.state::<BaseConf>().title_page.clone(),
        _ => None
    };
    res.and_then(|pth| {
        if pth.ends_with("$TitlePage.html"){
            return Some(include_str!("../titlepage.html").to_string());
        }
        std::fs::read_to_string(pth).ok()
    })
}

// #[tauri::command]
// pub fn get_projconf() -> Result<ProjConf,String> {
//   use std::fs::read_to_string;
//   let proj = read_to_string("ral.proj.yaml")
//       .or(read_to_string("ral.proj.yml"))
//       .map_err(|err| err.to_string())?;
//   str2yml(&proj).map_err(|err| err.to_string())
// }

// #[tauri::command]
// pub fn create_default_projconf() -> Result<(),String>{
//   use std::fs::File;
//   let proj_config = ProjConf::default();
//   let mut conf_file = File::create("ral.proj.yaml").map_err(|err|err.to_string())?;
//   conf_file.write_all(yml2str(&proj_config).unwrap().as_bytes()).map_err(|err| err.to_string())?;
//   Ok(())
// } 