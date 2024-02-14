// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use asn1_rs::{Any, Oid};
use polyproto::{PrivateKey, PublicKey, Signature, SignatureAlgorithm};

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
    key: Vec<u8>,
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
    type PublicKey = Ed25519PublicKey;
    type Signature = Ed25519Signature;
    fn to_bytes(&self) -> &[u8] {
        todo!()
    }

    fn pubkey(&self) -> Ed25519PublicKey {
        todo!()
    }

    fn sign(&self, data: &[u8]) -> Ed25519Signature {
        todo!()
    }
}

impl PublicKey<Ed25519> for Ed25519PublicKey {
    type Signature = Ed25519Signature;
    fn to_bytes(&self) -> &[u8] {
        todo!()
    }

    fn verify_signature(&self, signature: Ed25519Signature, data: &[u8]) -> bool {
        todo!()
    }
}
