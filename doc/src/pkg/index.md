# Package format

A Mangrove package is a zstandard-compressed tar archive with the following file structure:

```txt
example-package.mgve/
├── pkginfo
├── pkgfiles
└── <all included files>
```

The package data format is heavily inspired from the Pacman package manager.

## pkginfo

The `pkginfo` file is a serialized binary file which contains the following data:

| Key       | Example                                 | Description                                                                                                     |
| --------- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| pkgname   | `helloworld`                            | The name of the package. Can contain letters, numbers, dashes and underscores, but must start with a letter.    |
| pkgver    | `1.0.0`                                 | The version of the package. Must meet semver standard.                                                          |
| shortdesc | `A package to say hello world.`         | A short description, <80 chars.                                                                                 |
| longdesc  | `A package to say hello world.`         | A long description. Defaults to shortdesc if not found.                                                         |
| arch | `x86_64` | The architecture of the package. `uname -m` names are used here. |
| url | `https://example.com` | The website/URL of the software. |
| license | `GPL-3-or-later` | The SPDX-License-Identifier of the software. |
| groups | `example,test` | A list of groups the package is in. |
| depends   | `hello-world-data>=0.0.1,linux>=5.16.1` | A list of required dependencies. See [Specifying packages](../internals/pkgspec.md) for more info. |
| optdepends | `cool-thingy@1.0.0: for doing cool thing` | A newline-separated list of optional dependencies and strings to show to the user at install time. |
| provides | `hello-world@2.0.0` | A list of packages that this package provides the features of. |
| conflicts | `bad-program<=1.0.0` | A list of packages that this package cannot be installed alongside. |
| replaces | `old-program>=1.0.0` | A list of packages that this package replaces. |
| installed_size | `385723487` | The size in bytes of the package after installation. |
The above can be represented in json as following:

```json
{
    "pkgname": "helloworld",
    "pkgver": "1.0.0",
    "shortdesc": "A package to say hello world.",
    "arch": "x86_64",
    "url": "https://example.com",
    "license": "GPL-3-or-later",
    "groups": ["example", "test"],
    "depends": ["hello-world-data>=0.0.1", "linux>=5.16.1"],
    "optdepends": ["cool-thingy@1.0.0: for doing cool thing"],
    "provides": ["hello-world@2.0.0"],
    "conflicts": ["bad-program<=1.0.0"],
    "replaces": "old-program>=1.0.0",
    "installed_size": 385723487
}
```

The pkginfo file is a messagepack dictionary following the above structure. This allows for very small metadata files while still making it easy to quickly modify packages on the fly.
The above data is serialized as:

```txt
8d a7 70 6b 67 6e 61 6d 65 aa 68 65 6c 6c 6f 77 6f 72 6c 64 a6 70 6b 67 76 65 72 a5 31 2e 30 2e 30 a9 73 68 6f 72 74 64 65 73 63 bd 41 20 70 61 63 6b 61 67 65 20 74 6f 20 73 61 79 20 68 65 6c 6c 6f 20 77 6f 72 6c 64 2e a4 61 72 63 68 a6 78 38 36 5f 36 34 a3 75 72 6c b3 68 74 74 70 73 3a 2f 2f 65 78 61 6d 70 6c 65 2e 63 6f 6d a7 6c 69 63 65 6e 73 65 ae 47 50 4c 2d 33 2d 6f 72 2d 6c 61 74 65 72 a6 67 72 6f 75 70 73 92 a7 65 78 61 6d 70 6c 65 a4 74 65 73 74 a7 64 65 70 65 6e 64 73 92 b7 68 65 6c 6c 6f 2d 77 6f 72 6c 64 2d 64 61 74 61 3e 3d 30 2e 30 2e 31 ad 6c 69 6e 75 78 3e 3d 35 2e 31 36 2e 31 aa 6f 70 74 64 65 70 65 6e 64 73 91 d9 27 63 6f 6f 6c 2d 74 68 69 6e 67 79 40 31 2e 30 2e 30 3a 20 66 6f 72 20 64 6f 69 6e 67 20 63 6f 6f 6c 20 74 68 69 6e 67 a8 70 72 6f 76 69 64 65 73 91 b1 68 65 6c 6c 6f 2d 77 6f 72 6c 64 40 32 2e 30 2e 30 a9 63 6f 6e 66 6c 69 63 74 73 91 b2 62 61 64 2d 70 72 6f 67 72 61 6d 3c 3d 31 2e 30 2e 30 a8 72 65 70 6c 61 63 65 73 b2 6f 6c 64 2d 70 72 6f 67 72 61 6d 3e 3d 31 2e 30 2e 30 ae 69 6e 73 74 61 6c 6c 65 64 5f 73 69 7a 65 ce 16 fd ac 5f
```

Only 359 bytes!
