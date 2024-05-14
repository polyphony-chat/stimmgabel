// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::{SigningKey, VerifyingKey};
use polyproto::certs::PublicKeyInfo;
use polyproto::der::asn1::BitString;
use polyproto::signature::Signature;

use super::signature::SignatureEd25519;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKeyEd25519 {
    pub public_key: PublicKeyEd25519,
    key: SigningKey,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PublicKeyEd25519 {
    pub key: VerifyingKey,
}

impl polyproto::key::PublicKey<SignatureEd25519> for PublicKeyEd25519 {
    fn verify_signature(
        &self,
        signature: &SignatureEd25519,
        data: &[u8],
    ) -> Result<(), polyproto::errors::composite::PublicKeyError> {
        match self.key.verify_strict(data, signature.as_signature()) {
            Ok(_) => Ok(()),
            Err(_) => Err(polyproto::errors::composite::PublicKeyError::BadSignature),
        }
    }

    fn public_key_info(&self) -> polyproto::certs::PublicKeyInfo {
        PublicKeyInfo {
            algorithm: SignatureEd25519::algorithm_identifier(),
            public_key_bitstring: BitString::from_bytes(&self.key.to_bytes()).unwrap(),
        }
    }

    fn try_from_public_key_info(
        public_key_info: polyproto::certs::PublicKeyInfo,
    ) -> Result<Self, polyproto::errors::composite::ConversionError> {
        let mut key_vec = public_key_info.public_key_bitstring.raw_bytes().to_vec();
        key_vec.resize(32, 0);
        let signature_array: [u8; 32] = {
            let mut array = [0; 32];
            array.copy_from_slice(&key_vec[..]);
            array
        };
        match VerifyingKey::from_bytes(&signature_array) {
            Ok(key) => Ok(PublicKeyEd25519 { key }),
            Err(e) => Err(polyproto::errors::composite::ConversionError::InvalidInput(
                polyproto::errors::base::InvalidInput::Malformed(format!(
                    "Could not convert public key: {}",
                    e
                )),
            )),
        }
    }
}

impl polyproto::key::PrivateKey<crate::polyproto::signature::SignatureEd25519>
    for PrivateKeyEd25519
{
    type PublicKey = PublicKeyEd25519;

    fn pubkey(&self) -> &Self::PublicKey {
        &self.public_key
    }

    fn sign(&self, data: &[u8]) -> SignatureEd25519 {
        let signature = self.key.clone().sign(data);
        SignatureEd25519 { signature }
    }
}
