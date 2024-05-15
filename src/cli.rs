// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "stimmgabel")]
#[command(
    about = "A polyproto reference test implementation useful for verifying other implementations of the protocol."
)]
pub(crate) struct CliArguments {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Display the Ed25519 keys that should be used when supplying data to be verified. Keys are
    /// printed in PEM format
    Keys { key_choice: KeyChoice },
    /// Verify the well-formedness as well as the syntactical and cryptographical correctness of a
    /// given polyproto value
    Verify {
        /// The verification mode to use
        #[command(subcommand)]
        mode: StimmgabelMode,
    },
}

#[derive(Debug, Subcommand, PartialEq, Eq, Clone)]
pub(crate) enum StimmgabelMode {
    /// Verify a polyproto ID-Cert for its well-formedness and syntactical and cryptographical correctness
    IdCert {
        /// The value to verify
        value: String,
        #[arg(default_value_t = Format::Der, long = "encoding")]
        /// The format, in which the value is encoded
        encoding: Format,
        /// Who this certificate is supposed to be for
        target: Target,
    },
    /// The message to verify. Must be JSON encoded
    Message {
        /// The message to verify. Must be JSON encoded and match the following format:
        /// {
        ///     "message": "Any string",
        ///     "signature": "Base64 encoded signature",
        ///     "public_key": "Base64 encoded public key of the sender"
        /// }.
        /// The JSON may be minified and the order of the keys is unimportant
        value: String,
        /// Whether the message is to be verified from a home server or an actor perspective. Use
        /// the other party's private key to sign the message
        target: Target,
    },
    /// Verify a polyproto Id-CSR for its well-formedness and syntactical and cryptographical correctness
    IdCsr {
        /// The value to verify
        value: String,
        #[arg(default_value_t = Format::Der, long = "encoding")]
        /// The format, in which the value is encoded
        encoding: Format,
        /// Who this CSR is supposed to be for
        target: Target,
    },
    /// Verify a polyproto identity descriptor (IDD) for its well-formedness and syntactical correctness
    Idd {
        /// The value to verify
        value: String,
        #[arg(default_value_t = Format::Der, long = "encoding")]
        /// The format, in which the value is encoded
        encoding: Format,
        /// Who this IDD is supposed to be for
        target: Target,
    },
    /// Verify a relative polyproto identity descriptor name (RIDD) for its well-formedness and syntactical correctness
    RIdd {
        /// The value to verify
        value: String,
        #[arg(default_value_t = Format::Der, long = "encoding")]
        /// The format, in which the value is encoded
        encoding: Format,
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
pub(crate) enum Target {
    Actor,
    Homeserver,
}

impl ValueEnum for Target {
    fn value_variants<'a>() -> &'a [Self] {
        &[Target::Actor, Target::Homeserver]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Target::Actor => Some(PossibleValue::new("actor")),
            Target::Homeserver => Some(PossibleValue::new("homeserver")),
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Actor => write!(f, "actor"),
            Target::Homeserver => write!(f, "homeserver"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rdn {
    // TODO: ADD RDN VARIANTS
}
