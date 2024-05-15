// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::Flags;
use polyproto::certs::idcert::IdCert;
use polyproto::certs::idcsr::IdCsr;
use polyproto::errors::composite::ConversionError;
use polyproto::Constrained;

use crate::cli::{Format, StimmgabelMode, Target};
use crate::errors::ExitCode;
use crate::polyproto::keys::PublicKeyEd25519;
use crate::polyproto::signature::SignatureEd25519;

pub(crate) fn conversion_error_to_exit_code(error: ConversionError) -> i32 {
    match error {
        polyproto::errors::composite::ConversionError::ConstraintError(_) => {
            ExitCode::CONSTRAINT_VIOLATION.bits()
        }
        polyproto::errors::composite::ConversionError::InvalidInput(_)
        | polyproto::errors::composite::ConversionError::UnknownCriticalExtension { .. }
        | polyproto::errors::composite::ConversionError::ConstOidError(_) => {
            ExitCode::INVALID_INPUT.bits()
        }
        polyproto::errors::composite::ConversionError::DerError(_) => {
            ExitCode::GARBLED_INPUT.bits()
        }
        polyproto::errors::composite::ConversionError::IdCertError(idcert_error) => {
            return match idcert_error {
                polyproto::errors::composite::PublicKeyError::BadSignature => {
                    ExitCode::BAD_SIGNATURE.bits()
                }
                polyproto::errors::composite::PublicKeyError::BadPublicKeyInfo => {
                    ExitCode::BAD_PUBLIC_KEY.bits()
                }
            }
        }
    }
}

/// Verify the well-formedness as well as the syntactical and cryptographical correctness of a given
/// polyproto value. This function returns an exit code that can be used to signal the result of the
/// verification.
pub(crate) fn verify_input(mode: StimmgabelMode) -> i32 {
    // Due to how complex this function would be if we were to implement all the verification
    // logic here, we branch off here
    match mode {
        StimmgabelMode::IdCert {
            value,
            encoding,
            target,
        } => verify_certificate(&value, encoding, target),
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
    let certificate_result = match encoding {
        Format::Der => IdCert::<SignatureEd25519, PublicKeyEd25519>::from_der(value.as_bytes()),
        Format::Pem => IdCert::<SignatureEd25519, PublicKeyEd25519>::from_pem(value),
    };
    if let Err(error) = certificate_result {
        return conversion_error_to_exit_code(error);
    }
    let certificate = certificate_result.unwrap();
    let validation_result = match target {
        Target::Actor => certificate.validate_actor(),
        Target::Homeserver => certificate.validate_home_server(),
    };
    match validation_result {
        Ok(_) => 0,
        Err(error) => conversion_error_to_exit_code(error),
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
    let csr_result = match encoding {
        Format::Der => IdCsr::<SignatureEd25519, PublicKeyEd25519>::from_der(value.as_bytes()),
        Format::Pem => IdCsr::<SignatureEd25519, PublicKeyEd25519>::from_pem(value),
    };
    if let Err(error) = csr_result {
        return conversion_error_to_exit_code(error);
    }
    let csr = csr_result.unwrap();
    let validation_result = match target {
        Target::Actor => csr.validate_actor(),
        Target::Homeserver => csr.validate_home_server(),
    };
    match validation_result {
        Ok(_) => 0,
        Err(error) => conversion_error_to_exit_code(error),
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
