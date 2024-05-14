// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

bitflags! {
    pub struct ExitCode: i32 {
        const GARBLED_INPUT = 1 << 0;
        const INVALID_SPKI = 1 << 1;
        const MISMATCHED_ISSUER_FOR_SUBJECT = 1 << 2;
        const INVALID_CERTIFICATE_VERSION = 1 << 3;
        const INVALID_CERTIFICATE_SIGNATURE = 1 << 4;
        const UNKNOWN_SIGNATURE_ALGORITHM = 1 << 5;
        const INVALID_EXTENSIONS_FOR_ACTOR = 1 << 6;
        const INVALID_EXTENSIONS_FOR_HOMESERVER = 1 << 7;
    }
}
