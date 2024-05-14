// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(name = "stimmgabel")]
#[command(
    about = "A polyproto reference test implementation useful for verifying other implementations of the protocol."
)]
pub(crate) struct CliArguments {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub(crate) enum Commands {
    /// Display the Ed25519 keys that should be used when supplying data to be verified. Keys are
    /// printed in PEM format
    Keys { key_choice: KeyChoice },
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

/// The different keys that can be printed
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) enum KeyChoice {
    ActorPrivate,
    ActorPublic,
    HomeserverPrivate,
    HomeserverPublic,
}

impl ValueEnum for KeyChoice {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            KeyChoice::ActorPrivate,
            KeyChoice::ActorPublic,
            KeyChoice::HomeserverPrivate,
            KeyChoice::HomeserverPublic,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            KeyChoice::ActorPrivate => Some(PossibleValue::new("actor-private")),
            KeyChoice::ActorPublic => Some(PossibleValue::new("actor-public")),
            KeyChoice::HomeserverPrivate => Some(PossibleValue::new("homeserver-private")),
            KeyChoice::HomeserverPublic => Some(PossibleValue::new("homeserver-public")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Der,
    Pem,
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Format::Der, Format::Pem]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Format::Der => Some(PossibleValue::new("der")),
            Format::Pem => Some(PossibleValue::new("pem")),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Der => write!(f, "der"),
            Format::Pem => write!(f, "pem"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StimmgabelMode {
    ActorCertificate,
    HomeserverCertificate,
    Message,
    ActorCsr,
    HomeserverCsr,
    ActorDn,
    HomeserverDn,
    ActorRdn,
    HomeserverRdn,
}

impl ValueEnum for StimmgabelMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            StimmgabelMode::ActorCertificate,
            StimmgabelMode::HomeserverCertificate,
            StimmgabelMode::Message,
            StimmgabelMode::ActorCsr,
            StimmgabelMode::HomeserverCsr,
            StimmgabelMode::ActorDn,
            StimmgabelMode::HomeserverDn,
            StimmgabelMode::ActorRdn,
            StimmgabelMode::HomeserverRdn,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            StimmgabelMode::ActorCertificate => Some(PossibleValue::new("certificate-actor")),
            StimmgabelMode::HomeserverCertificate => {
                Some(PossibleValue::new("certificate-homeserver"))
            }
            StimmgabelMode::Message => Some(PossibleValue::new("message")),
            StimmgabelMode::ActorCsr => Some(PossibleValue::new("csr-actor")),
            StimmgabelMode::HomeserverCsr => Some(PossibleValue::new("csr-homeserver")),
            StimmgabelMode::ActorDn => Some(PossibleValue::new("dn-actor")),
            StimmgabelMode::HomeserverDn => Some(PossibleValue::new("dn-homeserver")),
            StimmgabelMode::ActorRdn => Some(PossibleValue::new("rdn-actor")),
            StimmgabelMode::HomeserverRdn => Some(PossibleValue::new("rdn-homeserver")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rdn {
    // TODO: ADD RDN VARIANTS
}
