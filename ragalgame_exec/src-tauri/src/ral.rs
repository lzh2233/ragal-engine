use std::io::{Seek, self};
use std::path::{PathBuf, Path};

use serde_json::{json, Value as JsonValue};

use tauri::async_runtime::RwLock;
use ragal_parser::GalmdReader;

#[tauri::command]
pub async fn read_exp(reader_state: tauri::State<'_, RwLock<GalmdReader>>, exp_lines: u64) -> Result<JsonValue, String> {
    let mut reader = reader_state.write().await;
    let mut res = Vec::new();
    let idx = reader.read_exp(&mut res, exp_lines).map_err(|err|{err.to_string()})?;
    Ok(json!({
        "offset": idx,
        "vector": res
    }))
}

use ragal_parser::RalGalScript;

#[tauri::command]
pub async fn get_info(state: tauri::State<'_, RwLock<GalmdReader>>) -> Result<RalGalScript, String> {
    let mut reader = state.write().await;
    reader.get_info().map_err(|err|err.to_string())
}

#[tauri::command]
pub async fn seek_from_start(state: tauri::State<'_, RwLock<GalmdReader>>, start: u64) -> Result<u64, String> {
    let mut reader = state.write().await;
    reader.seek(io::SeekFrom::Start(start)).map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn reader_new<R: tauri::Runtime>(window: tauri::Window<R>, path : &Path) -> Result<(), String> {
    use tauri::Manager;
    if !window.manage(RwLock::new(GalmdReader::open(path).map_err(|err|{err.to_string()})?)){
        let state = window.state::<RwLock<GalmdReader>>();
        let mut reader = state.write().await;
        reader.reopen(path).map_err(|err| err.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn open_file(pth : &Path) -> Result<String, String>{
    use std::fs::read_to_string;
    read_to_string(pth).or_else(|err| Err(err.to_string()))
}

#[tauri::command]
pub fn current_exe_dir() -> PathBuf{
    use std::env::current_dir;
    current_dir().unwrap()
}