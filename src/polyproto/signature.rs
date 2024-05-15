// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use ed25519_dalek::Signature as ed25519_dalek_Signature;
use polyproto::signature::Signature;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureEd25519 {
    pub(crate) signature: ed25519_dalek_Signature,
}

impl Signature for SignatureEd25519 {
    type Signature = ed25519_dalek_Signature;

    fn as_signature(&self) -> &Self::Signature {
        &self.signature
    }

    fn algorithm_identifier() -> polyproto::spki::AlgorithmIdentifierOwned {
        polyproto::spki::AlgorithmIdentifierOwned {
            // Unwrap is ok because the OID is hard-coded and cannot be invalid
            oid: polyproto::der::asn1::ObjectIdentifier::from_str("1.3.101.112").unwrap(),
            parameters: None,
        }
    }

    fn from_bitstring(signature: &[u8]) -> Self {
        let mut sig = [0u8; 64];
        sig.copy_from_slice(signature);
        SignatureEd25519 {
            signature: ed25519_dalek_Signature::from_bytes(&sig),
        }
    }
}

impl polyproto::spki::SignatureBitStringEncoding for SignatureEd25519 {
    fn to_bitstring(&self) -> polyproto::der::Result<polyproto::der::asn1::BitString> {
        polyproto::der::asn1::BitString::from_bytes(self.signature.to_bytes().as_slice())
    }
}
