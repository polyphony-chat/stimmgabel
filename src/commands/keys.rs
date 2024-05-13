// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli::KeyChoice;
use crate::*;

/// Prints the key specified by the user with the [KeyChoice] enum
pub(crate) fn print_key(choice: &KeyChoice) {
    let key = match choice {
        KeyChoice::ActorPrivate => ED25519_PRIVATE_ACTOR,
        KeyChoice::ActorPublic => ED25519_PUBLIC_ACTOR,
        KeyChoice::HomeserverPrivate => ED25519_PRIVATE_HOMESERVER,
        KeyChoice::HomeserverPublic => ED25519_PUBLIC_HOMESERVER,
    };
    println!("{}", key);
}
