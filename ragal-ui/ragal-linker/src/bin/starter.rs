#![windows_subsystem = "windows"]
use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct GalPlayerDll{
    build_and_run: fn(cfg_path: &str)
}

fn main(){
    let player_dll = unsafe { Container::<GalPlayerDll>::load(r"ragalRT\ragalgame_exec.dll").expect("运行时问题") };
    player_dll.build_and_run(r"ragal.cfg.yml");
}