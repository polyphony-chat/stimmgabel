use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;

mod ascii;
pub mod testcase;

lazy_static! {
    static ref CLI_ARGS: Cli = Cli::parse();
}

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
    std::env::set_var("RUST_LOG", if CLI_ARGS.verbose { "DEBUG" } else { "" });
    if CLI_ARGS.waves {
        ascii::ascii_waves()
    }
    match CLI_ARGS.mode {
        Mode::Client => client::run(CLI_ARGS.port),
        Mode::Server => server::run(CLI_ARGS.port).await,
    }
}
