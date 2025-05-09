use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Do Not Print New Line
    #[arg(short)]
    n: bool,

    text: String,
}


fn main() {
    let args = Args::parse();

    if !args.n {
        println!("{}", args.text)
    } else {
        print!("{}", args.text)
    }
}
