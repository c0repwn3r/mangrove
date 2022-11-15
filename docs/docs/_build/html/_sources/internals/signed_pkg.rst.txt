Signed Package Format
---------------------

.. note::
    This document refers to the Signed Package Format, which is Mangrove's encrypted file container. It is used to contain :ref:`package files <pkg_intro>`, which contain package data.
.. tip::
    Signed Packages, contrary to their name, can actually be used for any file, however they are engineered to be very difficult to decrypt, on purpose, and are therefore not recommended for that purpose.

Mangrove's "signed packages" are *actually* encrypted heavily using a combination of AES256 and Ed25519.

Rationale
---------

Why are we encrypting packages in the first place?
Mangrove was carefully planned out weeks before development even began. The decision to encrypt packages was made for several reasons:

- it makes it pretty much impossible for users to install packages that are tampered with by accident
    - by design tampered packages are unreadable, because the signature wont match, and therefore the derived encryption key wont either
- it makes corruptions immediately noticeable, without a possibility of system damage
     - as a corruption in the file would result in a signature mismatch, and therefore the wrong encryption key being derived, the resultant data would be garbage and there is an astronomically small chance that it will get anywhere past archive open

This decision has some downsides, but which have been deemed too small to consider an issue:

- a user needs to trust the signing key of a package before the package manager can even recognize it as a valid package
    - this is good practice anyway; users shouldn't be able to install unsigned packages easily
        - especially since mgve provides good tooling to very, very easily sign your packages, there is no situation other then malice where an unsigned package ever needs to be installed
- it can be difficult to inspect the contents of packages manually
    - this can be accomplished by "installing" the package to a local target directory, which will extract the files, while still verifying signatures and performing all normal safety checks
    - we very intentionally do not and never will provide a utility to convert an already signed package into an unsigned one. This defeats the purpose of signing packages in the first place, and opens it back up to tampering.

Anyways, that's enough theory. Into the technicalities!

Structure
---------

Signed packages follow a very specific binary structure to represent their data.
The binary starts off with the bytes ``4D 47 56 45``, which correspond to ``MGVE``, hex-encoded. This serves as the "magic" for signed packages - and can be used to quickly determine if the file we are working with has a chance of being a signed package.

.. list-table::
    :header-rows: 1

    * - field
      - value
      - description

    * - magic
      - 0x4d475645
      - 'MGVE' ascii, quickly identify possible package files

Next up is the signature length. It is one, byte, a ``u8``/``uint8_t`` which represents the signature's size in bytes. This was picked because ed25519 signatures are only 64 bytes and this allows for room for expansion with a later revision to the protocol.
After the signature length, is the signature itself. It is an unknown number of raw bytes, with length determined by ``s_len``.

.. list-table::
    :header-rows: 1

    * - field
      - value
      - description

    * - magic
      - 0x4d475645
      - 'MGVE' ascii, quickly identify possible package files

    * - s_len
      - 0x??
      - Describe the length, up to 255 bytes, of the following signature.

    * - s_dat
      - 0x?? * s_len
      - The data of the ed25519 signature of the package contents

We then include a null byte, which is the signature-data delimiter. It is an anchor point used to validate the preceding structure.

.. list-table::
    :header-rows: 1

    * - field
      - value
      - description

    * - magic
      - 0x4d475645
      - 'MGVE' ascii, quickly identify possible package files

    * - s_len
      - 0x??
      - Describe the length, up to 255 bytes, of the following signature.

    * - s_dat
      - 0x?? * s_len
      - The data of the ed25519 signature of the package contents

    * - s_sep
      - 0x00
      - Anchor the signature and the data

Next up is a 32-bit unsigned integer to put a length constraint on the actual encrypted data, followed by the encrypted data itself.
To cap it off, ``0x42`` is used as an ending delimiter.

.. list-table::
    :header-rows: 1

    * - field
      - value
      - description

    * - magic
      - 0x4d475645
      - 'MGVE' ascii, quickly identify possible package files

    * - s_len
      - 0x??
      - Describe the length, up to 255 bytes, of the following signature.

    * - s_dat
      - 0x?? * s_len
      - The data of the ed25519 signature of the package contents

    * - s_sep
      - 0x00
      - Anchor the signature and the data

    * - d_len
      - 0x????????
      - Describe the length, up to about 2.41 GB, of the following encrypted data.

    * - d_dat
      - 0x?? * d_len
      - The actual encrypted data of the package

    * - p_val
      - 0x42
      - Anchor the end of the package

And there you have it! That is the binary structure of a signed package.

How the data is encrypted
-------------------------

Upon getting data to encrypt and a Ed25519 PrivateKey, the implementation should use the PrivateKey to create a Ed25519 digital signature of the data.
It should then perform a sha256 hash on this signature, and use it as a key for a PKCS#7 padded AES-256 cipher.
This cipher is used to encrypt the package data.
The implementation should then put it into the above format, and return it to the caller.