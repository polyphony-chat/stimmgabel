# stimmgabel

polyproto reference test implementation used for verifying other implementations of the protocol.

## Verifying a message

polyproto does not dictate a specific format for messages. For this verification implementation, the following
format is used:

```json
{
    "message": "Any string",
    "signature": "Base64 encoded signature",
    "public_key": "Base64 encoded public key of the sender"
}
```

Minifying the JSON is allowed, and the order of the keys is not important.

## Cryptography and Safety

The cryptography, which includes keys and algorithmic implementations, supplied with this repository,
is ONLY to be used for testing purposes.

This software makes use of the ed25519-dalek crate to implement polyproto using ED25519 as a signature
algorithm. Please read up on the ed25519-dalek crate if you are interested in the safety guarantees made by this crate.

This software has not undergone any sort of security audit and, at the time of writing this, has
been written by one person who is not a cryptography expert. You MUST not rely on this software to be
"bulletproof".

The README is not done yet. So is the code. ;3