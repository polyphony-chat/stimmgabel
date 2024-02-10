use clap::{Parser, ValueEnum};

mod ascii;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, short)]
    mode: Mode,
    /// Port for the server to listen on, or for the client to connect to.
    #[arg(long, short, default_value_t = 4001)]
    port: u16,
    #[arg(long, short)]
    verbose: bool,
    #[arg(long, short)]
    waves: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Mode {
    Client,
    Server,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Cli::parse();
    std::env::set_var("RUST_LOG", if args.verbose { "DEBUG" } else { "" });
    if args.waves {
        ascii::ascii_waves()
    }
    match args.mode {
        Mode::Client => client::run(args.port),
        Mode::Server => server::run(args.port).await,
    }
}
