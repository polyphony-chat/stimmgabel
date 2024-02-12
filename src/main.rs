// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use clap::{Parser, ValueEnum};
use colored::Color;
use lazy_static::lazy_static;

mod ascii;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
pub mod testcase;

lazy_static! {
    static ref CLI_ARGS: Cli = Cli::parse();
    static ref COLORS_PASS: (Color, Color) = match CLI_ARGS.colorblind {
        true => (Color::BrightBlue, Color::Blue),
        false => (Color::BrightGreen, Color::Green),
    };
    static ref COLORS_FAIL: (Color, Color) = match CLI_ARGS.colorblind {
        true => (Color::BrightYellow, Color::Yellow),
        false => (Color::BrightRed, Color::Red),
    };
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short)]
    mode: Mode,
    /// Port for the server to listen on, or for the client to connect to.
    #[arg(long, short, default_value_t = 4001)]
    port: u16,
    #[arg(long, short)]
    verbose: bool,
    #[arg(long, short)]
    waves: bool,
    /// Changes the set of "fail/pass" colors to more colorblind-friendly ones.
    #[arg(long, default_value_t = std::env::var("COLORBLIND").is_ok())]
    colorblind: bool,
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
