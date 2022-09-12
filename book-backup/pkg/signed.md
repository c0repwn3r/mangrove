# Signed packages

An integral part of the Mangrove security system is package signing.
This system, detailed in this document, is designed to make it annoying to tamper with packages without altering the [trustcache](../internals/trustcache.md) on the target system.

This system was designed with the following requirements in mind:

- speed should be equal priority with security
- it should be easy to upgrade to more secure systems should they become avaliable
- removing signatures from already-signed packages should be difficult
- installing unsigned packages or packages from an untrusted source should require trustcache modification

## Structure

Signed mangrove packages are a custom binary format which wraps the normal unsigned package.
It is detailed below:

| field | size    | value      | description                                                    |
|:-----:|:-------:|:----------:|:--------------------------------------------------------------:|
| magic | 4       | 0x4e475645 | 'MGVE' ascii, used to make sure the package is actually signed |
| s_len | 1       | Any u8     | Length of the signature                                        |
| s_dat | `s_len` | Arbitrary  | Actual signature data                                          |
| s_sep | 1       | 0x00       | Signature/Data sentinel                                        |
| d_len | 4       | Any u32    | Length of the package data                                     |
| d_dat | `d_len` | Arbitrary  | Package data                                                   |
| p_val | 1       | 0x42       | End sentinel                                                   |

## Signature

The signature is an ed25519 signature, but this is changeable in future. It is the signature of the unencrypted package data (see Encryption below),
and is used to derive the encryption key.

## Encryption

To discourage tampering, the package data is encrypted using AES256. The key is derived from the signature via SHA256, such that simply removing the signature would result in not having the decryption key.
