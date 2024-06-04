// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod config;
mod ral;

use config::{
    custom_html,
    src_config,
    game_config,
    current_cfg_dir
};

use ral::{
    current_exe_dir,
    get_info,
    open_file,
    read_exp,
    seek_from_start,
    reader_new
};
use tauri::Manager;

use std::path::PathBuf;
use std::sync::OnceLock;

// pub use tauri::App;

// static AHDL : OnceLock<tauri::AppHandle> = OnceLock::new();
static AHDL : OnceLock<tauri::AppHandle> = OnceLock::new();
thread_local! {
    static  APP : std::cell::RefCell<Option<tauri::App>> = std::cell::RefCell::new(None);
}

fn error_message(txt: &str){
    use windows::{
        core::*,
        Win32::UI::WindowsAndMessaging::*
    };
    let s = format!("{}\0", txt).encode_utf16().collect::<Vec<_>>();
    let s = PCWSTR::from_raw(s.as_ptr() as *const u16);
    unsafe {
        
        MessageBoxW(None, s, w!("运行时错误"), MB_OK);
    }
}

fn _build(cfg_path: PathBuf) -> Result<tauri::App, String> {
    tauri::Builder::default()
        .setup(|app|{
            use config::StartBy;
            
            app.manage(cfg_path.parent().expect("配置文件路径错误").to_path_buf());
            let projcfg = config::ProjConf::open_or_default(cfg_path).map_err(|err|err.to_string())?;
            app.app_handle().start_by(projcfg).map_err(|err|err.to_string())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            custom_html,
            src_config,
            game_config,
            current_cfg_dir,

            current_exe_dir,
            get_info,
            open_file,
            read_exp,
            seek_from_start,
            reader_new
        ])
        .build(tauri::generate_context!())
        .map_err(|err| err.to_string())
} 

#[no_mangle]
pub fn build(cfg_path: PathBuf){
    match _build(cfg_path){
        Ok(app) => {
            let _ = AHDL.set(app.handle());
            APP.replace(Some(app));
        },
        Err(err) => error_message(&err)
    }
}

#[no_mangle]
pub fn build_and_run(cfg_path: &str) {
    match _build(cfg_path.into()) {
        Ok(app) => app.run(|_,_|{}),
        Err(err) => error_message(&err)
    } 
    
}

#[no_mangle]
pub fn run() {
    
    let Some(app) = APP.take()else{return};
    app.run(|_, _|{})
}

fn get_window(label: &str) -> Option<tauri::Window>{
    let apphandle = AHDL.get().unwrap();
    apphandle.get_window(label)
}

#[no_mangle]
pub fn enable_open_devtool(label: &str){
    
    let Some(window)  = get_window(label)else{return};
    let window = std::sync::Arc::new(window);
    let w = window.clone();
    window.listen("open-devtool", move|_|w.open_devtools());
}