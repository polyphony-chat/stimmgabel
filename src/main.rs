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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
    match args.mode {
        Mode::Client => client::run(args.port.unwrap_or(4001)),
        Mode::Server => server::run(args.port.unwrap_or(4001)).await,
    }
}
