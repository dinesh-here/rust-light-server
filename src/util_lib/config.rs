


use std::{env, path::PathBuf};

use clap::{Parser, ArgAction};


fn get_default_log_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("");
    path
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, default_value_t=7878)]
    pub port: i32,
    #[arg(short, long, action(ArgAction::SetTrue))]
    pub gzip: bool,

    #[arg(default_value=get_default_log_path().into_os_string())]
    pub path: PathBuf
    
}