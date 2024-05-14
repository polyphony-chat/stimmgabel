// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli::{Format, StimmgabelMode};

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// polyproto value. This function returns an exit code that can be used to signal the result of the
/// verification.
pub(crate) fn verify_input(mode: StimmgabelMode, value: &str, format: Format) -> i32 {
    // Due to how complex this function would be if we were to implement all the verification
    // logic here, we branch off here
    match mode {
        StimmgabelMode::ActorCertificate | StimmgabelMode::HomeserverCertificate => {
            verify_certificate(value, format)
        }
        StimmgabelMode::Message => verify_message(value, format),
        StimmgabelMode::ActorCsr | StimmgabelMode::HomeserverCsr => verify_csr(value, format),
        StimmgabelMode::ActorDn | StimmgabelMode::HomeserverDn => verify_dn(value, format),
        StimmgabelMode::ActorRdn | StimmgabelMode::HomeserverRdn => verify_rdn(value, format),
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// certificate value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_certificate(value: &str, format: Format) -> i32 {
    match format {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}

/// Verify the cryptographical correctness of a given
/// message value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_message(value: &str, format: Format) -> i32 {
    todo!()
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// CSR value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_csr(value: &str, format: Format) -> i32 {
    match format {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// DN value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_dn(value: &str, format: Format) -> i32 {
    match format {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// RDN value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_rdn(value: &str, format: Format) -> i32 {
    match format {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}
