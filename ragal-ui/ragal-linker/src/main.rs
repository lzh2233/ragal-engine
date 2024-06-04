#![windows_subsystem = "windows"]

use dlopen2::wrapper::{Container, WrapperApi};
use std::path::PathBuf;
use once_cell::sync::Lazy;


static GAL_PLAYER : Lazy<Container<GalPlayer>> = Lazy::new(||unsafe { Container::<GalPlayer>::load(r"ragalgame_exec.dll") }.unwrap());

#[derive(WrapperApi)]
struct GalPlayer{
    build : fn(cfg_path: PathBuf),
    run: fn(),
    build_and_run: fn(cfg_path: &str),
    // get_window: fn(app: &str) -> tauri::Window,
    enable_open_devtool: fn(label: &str)
}

struct CEX2Builder<'a>{
    run: Option<&'a str>,
    console_tool: bool,
}

struct CEX2<'a>{
    run_path: &'a str,
    console_tool: bool,
}


impl Default for CEX2Builder<'_> {
    fn default() -> Self {
        Self { run: None, console_tool: false }
    }
}

impl<'a> CEX2Builder<'a>{
    fn set_run_path(&mut self, path: &'a str) -> Option<&mut Self>{
        if self.run != None {
            return None
        }
        self.run = Some(path);
        Some(self)
    }

    fn enable_console(&mut self) -> &mut Self{
        self.console_tool = true;
        self
    }

    fn build(self) -> Result<CEX2<'a>, String>{
        match self.run {
            Some(run_path) => {
                Ok(CEX2{
                    run_path,
                    console_tool: self.console_tool,
                })
            },
            None => Err("no run-path!".into())
        }
    }

}

impl<'a> CEX2<'a> {
    fn run(self) {
        let player_dll = &GAL_PLAYER;
        player_dll.build(self.run_path.into());
        if self.console_tool {
            player_dll.enable_open_devtool("main");

        }
        player_dll.run();
    }
}

fn main(){
    use std::env;
    let t = env::args().collect::<Vec<_>>();
    let mut env_args = t.iter();
    let mut builder = CEX2Builder::default();
    while let Some(x) = env_args.next(){
        if x == "--run" {
            let Some(run_path) = env_args.next()else{break;};
            if !run_path.starts_with("--"){
                let Some(_) = builder.set_run_path(run_path)else{eprintln!("twice '--run' is error behave");return;};
            }else{
                eprintln!("Path parameters are required!");
                return;
            }
        }else if x == "--enable-devtool"{
            builder.enable_console();
        }
    }
    match builder.build() {
        Err(err) => eprintln!("{}", err),
        Ok(cex2) => cex2.run()
    }
}
