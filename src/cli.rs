// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub(crate) struct CliArguments {
    #[arg(short, long)]
    pub(crate) mode: StimmgabelMode,
    value: String,
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
