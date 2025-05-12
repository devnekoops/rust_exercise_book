use clap::{ArgAction, Parser};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config{
    #[arg(default_value = "-", action = ArgAction::Append)]
    filenames: Vec<String>,

    #[arg(short, long, default_value_t = 10)]
    lines: usize,

    #[arg(short, long, default_value = None)]
    bytes: Option<usize>


}

pub fn run(config: Config) -> MyResult<()>{
    println!("{:?}", config);
    let lines = config.lines;
    Ok(())
}

pub fn get_args() -> MyResult<Config>{
    let config = Config::parse();

    
    Ok(config)
}



fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(x) if x > 0 => Ok(x),
        _ => Err(From::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}