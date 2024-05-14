// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli::{Format, StimmgabelMode};

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// polyproto value. This function returns an exit code that can be used to signal the result of the
/// verification.
pub(crate) fn verify_input(mode: StimmgabelMode, value: &str, format: Format) -> i32 {
    todo!()
}
