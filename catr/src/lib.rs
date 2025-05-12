use std::error::Error;
use clap::{ArgAction, Parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(default_value = "-", action = ArgAction::Append)]
    files: Vec<String>,

    /// flags
    #[arg(short, long="number")]
    number_lines: bool,

    #[arg(short='b', long="number-nonblank")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()>{
    for filename in config.files {
        let mut line_num = 0;

        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                
                for line_result in file.lines(){
                    let line = line_result?;

                    if config.number_nonblank_lines {
                        if line.len() != 0 {
                            line_num += 1;
                            println!("{:>6}\t{}", line_num, line);
                        } else {
                            println!("");
                        }
                    } else if config.number_lines {
                        line_num += 1;
                        println!("{:>6}\t{}", line_num, line);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}