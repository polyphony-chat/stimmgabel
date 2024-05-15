# stimmgabel

Command-line utility implementation of polyproto, used for verifying other implementations of the protocol.

## Motivation

Getting all aspects right when implementing a protocol can be difficult - especially when the protocol
is new and does not have a lot of implementations to compare against. Stimmgabel (German for 'tuning fork')
is a command-line utility to use as a verification tool for other implementations of polyproto.

Since this is a binary and not a library, it is fairly straightforward to use in various programming languages,
by calling the binary with the desired arguments.

## Installation

Download the latest release from the [releases page](https://github.com/polyphony-chat/stimmgabel/releases).
Alternatively, using the Rust tool chain, you can clone this repository and build the binary yourself.

## Verifying a message

polyproto does not dictate a specific format for messages. For this verification implementation, the following
format is used, when passing a message to be verified:

```json
{
    "message": "Any string",
    "signature": "Base64 encoded signature",
    "public_key": "Base64 encoded public key of the sender"
}
```

Minifying the JSON is allowed, and the order of the keys is not important.

## Cryptography and Safety

polyproto does not specify a signature algorithm. For two implementations to be compatible, they must offer
an overlapping set of signature algorithms. Stimmgabel only supports ED25519, using the ed25519-dalek crate,
as ED25519 is the currently recommended signature algorithm for polyproto.

> [!IMPORTANT]
> This repository's cryptography, including keys and algorithms, is strictly for testing purposes.
> It uses the ed25519-dalek crate to implement polyproto with ED25519 as a signature algorithm.
> Learn about the safety guarantees of the ed25519-dalek crate for more information.
> This software has not received a security audit. No guarantees about its safety can be made.
