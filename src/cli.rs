// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub(crate) struct CliArguments {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub(crate) enum Commands {
    /// Display the Ed25519 keys that should be used when supplying data to be verified
    Key {
        /// Print the private key actors should use to generate data to be verified using stimmgabel
        #[arg(long = "actor")]
        actor_private_key: bool,
        /// Print the public key of the stimmgabel "virtual home server" that should be used to
        /// generate data to be verified using stimmgabel
        #[arg(long = "homeserver")]
        home_server_public_key: bool,
    },
    /// Verify the well-formedness as well as the syntactical and cryptographical correctness of a
    /// given polyproto value
    Verify {
        /// Which verification mode to use
        mode: StimmgabelMode,
        /// The value to verify
        value: String,
        /// The format, in which the value is encoded
        #[arg(default_value_t = Format::Der, long = "format")]
        format: Format,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Json,
    Der,
    Pem,
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Format::Json, Format::Der, Format::Pem]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Format::Json => Some(clap::builder::PossibleValue::new("json")),
            Format::Der => Some(clap::builder::PossibleValue::new("der")),
            Format::Pem => Some(clap::builder::PossibleValue::new("pem")),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Json => write!(f, "json"),
            Format::Der => write!(f, "der"),
            Format::Pem => write!(f, "pem"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StimmgabelMode {
    Certificate,
    Message,
    Csr,
    Dn,
    Rdn,
}

impl ValueEnum for StimmgabelMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            StimmgabelMode::Certificate,
            StimmgabelMode::Message,
            StimmgabelMode::Csr,
            StimmgabelMode::Dn,
            StimmgabelMode::Rdn,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            StimmgabelMode::Certificate => Some(clap::builder::PossibleValue::new("certificate")),
            StimmgabelMode::Message => Some(clap::builder::PossibleValue::new("message")),
            StimmgabelMode::Csr => Some(clap::builder::PossibleValue::new("csr")),
            StimmgabelMode::Dn => Some(clap::builder::PossibleValue::new("dn")),
            StimmgabelMode::Rdn => Some(clap::builder::PossibleValue::new("rdn")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rdn {
    IdentityDescriptor,
    SessionId,
    SerialNumber,
    AlgorithmIdentifier,
    Signature,
    Validity,
    SubjectPublicKeyInfo,
    Extensions,
}
