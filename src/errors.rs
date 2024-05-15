// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;

bitflags! {
    pub struct ExitCode: i32 {
        const GARBLED_INPUT = 1 << 0;
        const INVALID_INPUT = 1 << 1;
        const CONSTRAINT_VIOLATION = 1 << 2;
        const BAD_SIGNATURE = 1 << 3;
        const BAD_PUBLIC_KEY = 1 << 4;
    }
}
