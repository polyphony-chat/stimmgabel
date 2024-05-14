# stimmgabel

polyproto reference test implementation used for verifying other implementations of the protocol.

## Cryptography and Safety

The cryptography, which includes keys and algorithmic implementations, supplied with this repository, is ONLY to be used for testing purposes. 

This software makes use of the ed25519-dalek crate to implement polyproto using ED25519 as a signature algorithm. Please read up on the ed25519-dalek crate if you are interested in the safety guarantees made by this crate. 

This software has not undergone any sort of security audit and, at the time of writing this, has been written by one person who is not a cryptography expert. You MUST not rely on this software to be "bullet proof".


The README is not done yet. So is the code. ;3