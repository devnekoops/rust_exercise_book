use std::error::Error;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(default_value = "-", action = ArgAction::Append)]
    files: Vec<String>,

    /// flags
    #[arg(short, long)]
    number_lines: bool,

    #[arg(short='b', long)]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()>{
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    println!("OK");
    Ok(config)
}