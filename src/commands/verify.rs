// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli::{Format, StimmgabelMode, Target};

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// polyproto value. This function returns an exit code that can be used to signal the result of the
/// verification.
pub(crate) fn verify_input(mode: StimmgabelMode) -> i32 {
    // Due to how complex this function would be if we were to implement all the verification
    // logic here, we branch off here
    match mode {
        StimmgabelMode::IdCert {
            value,
            encoding: format,
            target,
        } => verify_certificate(&value, format, target),
        StimmgabelMode::Message { value } => verify_message(&value),
        StimmgabelMode::IdCsr {
            value,
            encoding,
            target,
        } => verify_csr(&value, encoding, target),
        StimmgabelMode::Idd {
            value,
            encoding,
            target,
        } => verify_dn(&value, encoding, target),
        StimmgabelMode::RIdd { value, encoding } => verify_rdn(&value, encoding),
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// certificate value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_certificate(value: &str, encoding: Format, target: Target) -> i32 {
    match encoding {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}

/// Verify the cryptographical correctness of a given
/// message value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_message(value: &str) -> i32 {
    todo!()
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// CSR value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_csr(value: &str, encoding: Format, target: Target) -> i32 {
    match encoding {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// DN value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_dn(value: &str, encoding: Format, target: Target) -> i32 {
    match encoding {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// RDN value. This function returns an exit code that can be used to signal the result of the
/// verification.
fn verify_rdn(value: &str, encoding: Format) -> i32 {
    match encoding {
        Format::Der => todo!(),
        Format::Pem => todo!(),
    }
}
