// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

static ED25519_PRIVATE_ACTOR: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIF56bBgJn5rIbW+i36Ssi3D+Q2wJ8/Ev513RQ+ToYDPH
-----END PRIVATE KEY-----";
static ED25519_PUBLIC_ACTOR: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAmZF4EPAUSZflbq+9Q+aCwCwpSW2z/zPR8HWKuJSy7VA=
-----END PUBLIC KEY-----";
static ED25519_PRIVATE_HOMESERVER: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIFVSih9uk5DzhIphun+vHWCmcBwYLNox2hm5up4JkCso
-----END PRIVATE KEY-----";
static ED25519_PUBLIC_HOMESERVER: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAy7tqf5pG2XTJg2zh451RUr2rp02Nl7E1/k4LU+UzJeE=
-----END PUBLIC KEY-----";

use clap::Parser;
use cli::{CliArguments, Commands};

pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod errors;
pub(crate) mod polyproto;

fn main() {
    // Parse arguments, then choose the correct command to run
    let args = CliArguments::parse();
    match args.command {
        Commands::Keys { key_choice } => {
            commands::keys::print_key(&key_choice);
            std::process::exit(0);
        }
        Commands::Verify {
            mode,
            value,
            format,
        } => commands::verify::verify_input(mode, &value, format),
    }
}
