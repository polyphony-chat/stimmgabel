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

lazy_static! {
    pub static ref ED25519_PRIVATE_ACTOR_KEY: SigningKey =
        SigningKey::from_pkcs8_pem(ED25519_PRIVATE_ACTOR).unwrap();
    pub static ref ED25519_PUBLIC_ACTOR_KEY: VerifyingKey =
        VerifyingKey::from_public_key_pem(ED25519_PUBLIC_ACTOR).unwrap();
    pub static ref ED25519_PRIVATE_HOMESERVER_KEY: SigningKey =
        SigningKey::from_pkcs8_pem(ED25519_PRIVATE_HOMESERVER).unwrap();
    pub static ref ED25519_PUBLIC_HOMESERVER_KEY: VerifyingKey =
        VerifyingKey::from_public_key_pem(ED25519_PUBLIC_HOMESERVER).unwrap();
}

use ::polyproto::spki::DecodePublicKey;
use clap::Parser;
use cli::{CliArguments, Commands};
use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{SigningKey, VerifyingKey};
use lazy_static::lazy_static;

pub(crate) mod cli;
pub(crate) mod commands;
pub mod errors;
pub(crate) mod polyproto;

fn main() {
    // Parse arguments, then choose the correct command to run
    let args = CliArguments::parse();
    match args.command {
        Commands::Keys { key_choice } => {
            commands::keys::print_key(&key_choice);
            std::process::exit(0);
        }
        Commands::Verify { mode } => std::process::exit(commands::verify::verify_input(mode)),
    }
}
