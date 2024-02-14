// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use asn1_rs::{Any, Oid};
use polyproto::{PrivateKey, PublicKey, Signature, SignatureAlgorithm};

#[derive(Clone, Debug, PartialEq)]
pub struct Ed25119 {
    oid: Oid<'static>,
    parameters: Option<Any<'static>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ed25119Signature {
    algorithm: Ed25119,
    signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Ed25119PrivateKey {
    pub_key: Ed25119PublicKey,
    key: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Ed25119PublicKey {
    key: Vec<u8>,
}

impl SignatureAlgorithm for Ed25119 {
    fn oid(&self) -> Oid {
        self.oid.clone()
    }

    fn parameters(&self) -> Option<Any<'_>> {
        self.parameters.clone()
    }

    fn name(&self) -> &str {
        "ED25119"
    }
}

impl Signature<Ed25119> for Ed25119Signature {
    fn signature(&self) -> &[u8] {
        &self.signature
    }

    fn algorithm(&self) -> Ed25119 {
        self.algorithm.clone()
    }
}

impl PrivateKey<Ed25119> for Ed25119PrivateKey {
    type PublicKey = Ed25119PublicKey;
    type Signature = Ed25119Signature;
    fn to_bytes(&self) -> &[u8] {
        todo!()
    }

    fn pubkey(&self) -> Ed25119PublicKey {
        todo!()
    }

    fn sign(&self, data: &[u8]) -> Ed25119Signature {
        todo!()
    }
}

impl PublicKey<Ed25119> for Ed25119PublicKey {
    type Signature = Ed25119Signature;
    fn to_bytes(&self) -> &[u8] {
        todo!()
    }

    fn verify_signature(&self, signature: Ed25119Signature, data: &[u8]) -> bool {
        todo!()
    }
}
