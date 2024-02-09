use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    mode: Mode,
    #[arg(long, short)]
    port: Option<u16>,
}

#[derive(Clone, Copy, ValueEnum)]
enum Mode {
    Client,
    Server,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
}
