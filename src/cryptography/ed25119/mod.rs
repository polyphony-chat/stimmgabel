// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use asn1_rs::{oid, Any, Oid};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use polyproto::{PrivateKey, PublicKey, Signature, SignatureAlgorithm};
use rand::prelude::{CryptoRng, RngCore};
use rand::rngs::OsRng;

#[derive(Clone, Debug, PartialEq)]
pub struct Ed25519 {
    oid: Oid<'static>,
    parameters: Option<Any<'static>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ed25519Signature {
    algorithm: Ed25519,
    signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Ed25519PrivateKey {
    pub_key: Ed25519PublicKey,
    key: SigningKey,
}

#[derive(Clone, Debug)]
pub struct Ed25519PublicKey {
    key: Vec<u8>,
}

impl SignatureAlgorithm for Ed25519 {
    fn oid(&self) -> Oid {
        self.oid.clone()
    }

    fn parameters(&self) -> Option<Any<'_>> {
        self.parameters.clone()
    }

    fn name(&self) -> &str {
        "Ed25519"
    }
}

impl Signature<Ed25519> for Ed25519Signature {
    fn signature(&self) -> &[u8] {
        &self.signature
    }

    fn algorithm(&self) -> Ed25519 {
        self.algorithm.clone()
    }
}

impl PrivateKey<Ed25519> for Ed25519PrivateKey {
    type PrivateKey = SigningKey;
    type PublicKey = Ed25519PublicKey;
    type Signature = Ed25519Signature;

    fn pubkey(&self) -> Ed25519PublicKey {
        todo!()
    }

    fn sign(&self, data: &[u8]) -> Ed25519Signature {
        let interim_signature = self.key().sign(data);
        Ed25519Signature {
            algorithm: Ed25519 {
                oid: oid!(1.3.101 .112),
                parameters: None,
            },
            signature: interim_signature.to_vec(),
        }
    }

    fn key(&self) -> &Self::PrivateKey {
        &self.key
    }

    fn key_mut(&mut self) -> &mut Self::PrivateKey {
        todo!()
    }
}

impl PublicKey<Ed25519> for Ed25519PublicKey {
    type PublicKey = VerifyingKey;
    type Signature = Ed25519Signature;

    fn verify_signature(&self, signature: Ed25519Signature, data: &[u8]) -> bool {
        todo!()
    }

    fn key(&self) -> &Self::PublicKey {
        todo!()
    }

    fn key_mut(&mut self) -> &mut Self::PublicKey {
        todo!()
    }
}

impl Ed25519PrivateKey {
    pub fn gen_keypair<R>(csprng: &mut R) -> Ed25519PrivateKey
    where
        R: CryptoRng + RngCore,
    {
        let signing_key: SigningKey = SigningKey::generate(csprng);
        Ed25519PrivateKey {
            pub_key: Ed25519PublicKey {
                key: signing_key.verifying_key().to_bytes().to_vec(),
            },
            key: signing_key,
        }
    }
}

#[test]
fn test() {
    let mut csprng = OsRng;
    let priv_key = Ed25519PrivateKey::gen_keypair(&mut csprng);
    println!("Private Key is: {:?}", priv_key.key.to_bytes());
    println!("Public Key is: {:?}", priv_key.pub_key.key);
    println!();

    let message_unsigned = "hi my name is flori".as_bytes();
    let signature = priv_key.sign(message_unsigned);
    println!(
        "Signature of the message \"{}\": {:?}",
        String::from_utf8_lossy(message_unsigned),
        signature.signature()
    )
}
