// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ed25519_dalek::VerifyingKey;
use log::*;
use polyproto::certs::idcert::IdCert;
use polyproto::certs::idcsr::IdCsr;
use polyproto::certs::PublicKeyInfo;
use polyproto::der::asn1::BitString;
use polyproto::errors::composite::ConversionError;
use polyproto::key::PublicKey;
use polyproto::signature::Signature;

use crate::cli::{Format, StimmgabelMode, Target};
use crate::errors::ExitCode;
use crate::polyproto::keys::PublicKeyEd25519;
use crate::polyproto::message::Message;
use crate::polyproto::signature::SignatureEd25519;
use crate::{ED25519_PUBLIC_ACTOR_KEY, ED25519_PUBLIC_HOMESERVER_KEY};

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
            match idcert_error {
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
    match ED25519_PUBLIC_HOMESERVER_KEY.verify_strict(
        &certificate.clone().to_der().unwrap(),
        certificate.signature.as_signature(),
    ) {
        Ok(_) => (),
        Err(_) => return ExitCode::BAD_SIGNATURE.bits(),
    }
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
    let message_result: Result<Message, serde_json::Error> = serde_json::from_str(value);
    if message_result.is_err() {
        return ExitCode::INVALID_INPUT.bits();
    }
    let message = message_result.unwrap();
    let signature = SignatureEd25519::from_bytes(message.signature.as_bytes());
    let public_key = match PublicKeyEd25519::try_from_public_key_info(PublicKeyInfo {
        algorithm: SignatureEd25519::algorithm_identifier(),
        public_key_bitstring: {
            let unused_bits = message.public_key.len() % 8;
            BitString::new(unused_bits as u8, message.public_key).unwrap()
        },
    }) {
        Ok(key) => key,
        Err(e) => return conversion_error_to_exit_code(e),
    };
    let verification_result = public_key.verify_signature(&signature, value.as_bytes());
    match verification_result {
        Ok(_) => 0,
        Err(error) => match error {
            polyproto::errors::composite::PublicKeyError::BadSignature => {
                ExitCode::BAD_SIGNATURE.bits()
            }
            polyproto::errors::composite::PublicKeyError::BadPublicKeyInfo => {
                ExitCode::BAD_PUBLIC_KEY.bits()
            }
        },
    }
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
    let verifying_key = match target {
        Target::Actor => ED25519_PUBLIC_ACTOR_KEY.to_bytes(),
        Target::Homeserver => ED25519_PUBLIC_HOMESERVER_KEY.to_bytes(),
    };
    match VerifyingKey::from_bytes(&verifying_key)
        .unwrap()
        .verify_strict(&csr.signature_data().unwrap(), csr.signature.as_signature())
    {
        Ok(_) => {
            debug!("Signature verification successful")
        }
        Err(e) => {
            error!("Signature verification failed: {}", e);
            return ExitCode::BAD_SIGNATURE.bits();
        }
    }
    let validation_result = match target {
        Target::Actor => csr.validate_actor(),
        Target::Homeserver => csr.validate_home_server(),
    };
    match validation_result {
        Ok(_) => 0,
        Err(error) => conversion_error_to_exit_code(error),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use polyproto::certs::capabilities::Capabilities;
    use polyproto::certs::idcsr::IdCsr;
    use polyproto::RdnSequence;

    use crate::polyproto::keys::{PrivateKeyEd25519, PublicKeyEd25519};
    use crate::polyproto::signature::SignatureEd25519;

    use super::verify_csr;

    #[test]
    fn verify_home_server_signed_actor_csr() {
        env_logger::try_init().unwrap_or(());
        let signing_key =
            ed25519_dalek::SigningKey::from_bytes(crate::ED25519_PRIVATE_ACTOR_KEY.as_bytes());
        let verifying_key =
            ed25519_dalek::VerifyingKey::from_bytes(crate::ED25519_PUBLIC_ACTOR_KEY.as_bytes())
                .unwrap();
        let public_key = PublicKeyEd25519 { key: verifying_key };
        let private_key = PrivateKeyEd25519 {
            public_key: public_key.clone(),
            key: signing_key,
        };
        let actor_cert_csr =
            IdCsr::<SignatureEd25519, PublicKeyEd25519>::new(&RdnSequence::from_str("CN=flori,DC=www,DC=polyphony,DC=chat,UID=flori@polyphony.chat,uniqueIdentifier=client1").unwrap(), &private_key, &Capabilities::default_actor()).unwrap();
        let exit_code = verify_csr(
            actor_cert_csr
                .to_pem(polyproto::der::pem::LineEnding::LF)
                .unwrap()
                .as_str(),
            crate::cli::Format::Pem,
            crate::cli::Target::Actor,
        );
        assert_eq!(exit_code, 0)
    }

    #[test]
    fn other_key_cannot_pass_verification_csr() {
        env_logger::try_init().unwrap_or(());
        let mut csprng = rand::rngs::OsRng;
        // Generate a key pair
        let private_key = PrivateKeyEd25519::gen_keypair(&mut csprng);
        let actor_cert_csr =
            IdCsr::<SignatureEd25519, PublicKeyEd25519>::new(&RdnSequence::from_str("CN=flori,DC=www,DC=polyphony,DC=chat,UID=flori@polyphony.chat,uniqueIdentifier=client1").unwrap(), &private_key, &Capabilities::default_actor()).unwrap();
        let exit_code = verify_csr(
            actor_cert_csr
                .to_pem(polyproto::der::pem::LineEnding::LF)
                .unwrap()
                .as_str(),
            crate::cli::Format::Pem,
            crate::cli::Target::Actor,
        );
        assert_ne!(exit_code, 0)
    }
}
