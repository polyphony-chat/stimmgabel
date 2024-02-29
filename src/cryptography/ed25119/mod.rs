// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use der::asn1::BitString;
use ed25519_dalek::{Signature as Ed25519DalekSignature, Signer, SigningKey, VerifyingKey};
use polyproto::key::{PrivateKey, PublicKey};
use polyproto::signature::Signature;
use rand::rngs::OsRng;
use spki::{AlgorithmIdentifierOwned, ObjectIdentifier, SignatureBitStringEncoding};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ed25519Signature {
    signature: Ed25519DalekSignature,
    algorithm: AlgorithmIdentifierOwned,
}

impl Signature for Ed25519Signature {
    type Signature = Ed25519DalekSignature;

    fn as_signature(&self) -> &Self::Signature {
        &self.signature
    }

    fn as_algorithm_identifier() -> AlgorithmIdentifierOwned {
        AlgorithmIdentifierOwned {
            oid: ObjectIdentifier::new_unwrap("1.3.101.112"),
            parameters: None,
        }
    }
}

impl SignatureBitStringEncoding for Ed25519Signature {
    fn to_bitstring(&self) -> der::Result<der::asn1::BitString> {
        BitString::from_bytes(&self.as_signature().to_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ed25519PrivateKey {
    public_key: Ed25519PublicKey,
    key: SigningKey,
}

impl PrivateKey<Ed25519Signature> for Ed25519PrivateKey {
    type PublicKey = Ed25519PublicKey;

    fn pubkey(&self) -> &Self::PublicKey {
        &self.public_key
    }

    fn sign(&self, data: &[u8]) -> Ed25519Signature {
        let signature = self.key.sign(data);
        Ed25519Signature {
            signature,
            algorithm: Ed25519Signature::as_algorithm_identifier(),
        }
    }

    fn to_bitstring(&self) -> Result<BitString, der::Error> {
        BitString::from_bytes(self.key.as_bytes())
    }
}

impl Ed25519PrivateKey {
    pub fn gen_keypair(csprng: &mut OsRng) -> Self {
        let key = SigningKey::generate(csprng);
        let public_key = Ed25519PublicKey {
            key: key.verifying_key(),
        };
        Self { public_key, key }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ed25519PublicKey {
    key: VerifyingKey,
}

impl PublicKey<Ed25519Signature> for Ed25519PublicKey {
    type Error = Error;

    fn verify_signature(
        &self,
        signature: &Ed25519Signature,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        match self.key.verify_strict(data, signature.as_signature()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SignatureVerification),
        }
    }

    fn to_bitstring(&self) -> Result<BitString, der::Error> {
        BitString::from_bytes(self.key.as_bytes())
    }
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("The signature failed to verify")]
    SignatureVerification,
}

#[cfg(test)]
mod test {
    use polyproto::key::{PrivateKey, PublicKey};
    use polyproto::signature::Signature;

    use crate::cryptography::ed25119::Ed25519PrivateKey;

    #[test]
    fn test() {
        let mut csprng = rand::rngs::OsRng;
        let priv_key = Ed25519PrivateKey::gen_keypair(&mut csprng);
        println!("Private Key is: {:?}", priv_key.key.to_bytes());
        println!("Public Key is: {:?}", priv_key.public_key.key.to_bytes());
        println!();

        let message_unsigned = "hi my name is flori".as_bytes();
        let signature = priv_key.sign(message_unsigned);
        println!(
            "Signature of the message \"{}\": {:?}",
            String::from_utf8_lossy(message_unsigned),
            signature.as_signature().to_bytes()
        );

        println!(
            "Is the signature valid? {}",
            priv_key
                .public_key
                .verify_signature(&signature, message_unsigned)
                .is_ok()
        );

        println!(
            "Trying again with different data. The result is: {}",
            priv_key
                .pubkey()
                .verify_signature(
                    &signature,
                    format!("{} ", String::from_utf8_lossy(message_unsigned)).as_bytes()
                )
                .is_ok()
        )
    }
}
