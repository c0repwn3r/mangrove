# Package Format
Mangrove uses a custom package format built from the ground up to meet the [Mangrove Design Principles](../internals.md).

> Note: This details the unsigned package format, which is normally contained inside the Signed Package Format. For more information on encrypted packages, see [signed packages](signed_pkg.md)

## Naming Convention
Mangrove package files all follow a common naming convention:
`<pkgbase>_<pkgver>_<arch>.mgve`
This is shared between encrypted and unencrypted packages. The package manager can automatically differentiate between the two because of the radically different structure of both formats.

## Outer Container
Unencrypted mangrove packages are stored in a Zlib-compressed tar archive, following the below structure:
```
test_0.0.1_amd64.mgve/
├─ pkginfo
├─ <package contents>
```
`<package contents>` contains the folder and file structure of the installed package, with the exception of symlinks. Symlinks are stored in the pkginfo file, and not placed inside the package to save space.

## pkginfo
`pkginfo` is, as the name suggests, the Package Information file. As with most other serialized binary files in Mangrove, it is a MessagePack-encoded data structure, which is defined as follows:
<details>
    <summary>Rust implementation</summary>

```rust
//
// Package
/// Responsible for representing all data about a package
//
#[derive(Serialize, Deserialize, Debug, PartialEq)] // Allow serde to do its magic
pub struct Package {
    pub pkgname: String,                 // package name: String (required)
    pub pkgver: Version,                 // package version: semver (required)
    pub shortdesc: String,               // Short description: String (required)
    pub longdesc: Option<String>,        // Long description: String (optional)
    pub arch: Architecture,              // Architecture: Architecture (required)
    pub url: Option<String>,             // URL: String (optional)
    pub license: Option<String>,         // License: String (optional)
    pub groups: Option<Vec<String>>,     // Groups: List of String (optional)
    pub depends: Option<Vec<PkgSpec>>,   // Depends: List of PkgSpec (optional)
    pub optdepends: Option<Vec<String>>, // Optional Depends: List of String (optional)
    pub provides: Option<Vec<PkgSpec>>,  // Provides: List of PkgSpec (optional)
    pub conflicts: Option<Vec<PkgSpec>>, // Conflicts: List of PkgSpec (optional)
    pub replaces: Option<Vec<PkgSpec>>,  // Replaces: List of PkgSpec (optional)
    pub installed_size: usize,           // Installed Size: integer (required)
    pub pkgcontents: PackageContents,    // Package Contents: PackageContents (required)
}

// get_pkg_filename
/// Utility function to get the filename for a Package
//
pub fn get_pkg_filename(package: &Package) -> String {
// pkgname_1.0.0-alpha.1+3423432_x86_64.mgve
format!(
"{}_{}_{}.mgve",
package.pkgname,
package.pkgver,
arch_str(&package.arch)
)
}

// PkgSpec
/// Represents a package specification (ie `test-package>=1`)
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PkgSpec {
pub pkgname: String,
pub version: VersionReq,
}

// PackageContents
/// Represents the contents of a package
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageContents {
pub folders: Option<Vec<PackageFolder>>,
pub files: Option<Vec<PackageFile>>,
pub links: Option<Vec<PackageLink>>,
}

// PackageFolder
/// Represents a folder a package creates, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFolder {
pub name: String,
pub mtime: usize,
pub installpath: String,
pub meta: FileMetadata,
}

// PackageFile
/// Represents a file inside a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFile {
pub name: String,
pub sha256: String,
pub meta: FileMetadata,
pub mtime: usize,
pub installpath: String,
}

// PackageLink
/// Represents a symbolic link that should be created by a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageLink {
pub file: String,
pub mtime: usize,
pub target: String,
}

// FileMetadata
/// Represents metadata on a file or folder
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileMetadata {
pub owner: usize,
pub group: usize,
pub permissions: usize,
}
```
</details>


For non-programmers, here is a convenient table representing the entire pkginfo file:

| Field          | Optional | Type            | Description                                                                                                                           | Example                                      |
|----------------|----------|-----------------|---------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------|
| pkgname        | no       | String          | The name of the package                                                                                                               | test                                         |
| pkgver         | no       | Version         | The version of the package                                                                                                            | 0.1.1                                        |
| shortdesc      | no       | String          | A short description of the package                                                                                                    | Says hello.                                  |
| longdesc       | yes      | String          | A longer description of the package                                                                                                   | It says hello when you run the hello binary. |
| arch           | no       | Architecture    | The architecture of the package                                                                                                       | amd64                                        |
| url            | yes      | String          | An optional URL to the homepage of this package                                                                                       | https://mgve.cc                              |
| license        | yes      | String          | The SPDX license identifier of the package                                                                                            | GNU-GPL-3.0-or-later                         |
| groups         | yes      | Vec<String>     | A list of groups this package is a part of.                                                                                           | ["group1", "group2"]                         |
| depends        | yes      | Vec<PkgSpec>    | A list of packages that this package depends on.                                                                                      | ["other-package>=1.0.0"]                     |
| optdepends     | yes      | Vec<PkgSpec>    | A list of packages that this package optionally depends on.                                                                           | ["optional-package>=1.0.0"]                  |
| provides       | yes      | Vec<PkgSpec>    | A list of other packages that this package provides the funtionality of. Generally, this means it has the same binaries or libraries. | ["other-package"]                            |
| conflicts      | yes      | Vec<PkgSpec>    | A list of other packages that this package cannot be installed alongside.                                                             | ["bad-package"]                              |
| replaces       | yes      | Vec<PkgSpec>    | A list of other packages that this package replaces.                                                                                  | ["old-package"]                              |
| installed_size | no       | usize           | The total installed size of this package.                                                                                             | 16387                                        |
| pkgcontents    | no       | PackageContents | An enumeration of the contents of this package, their permissions, and where they should be installed to.                             | See below.                                   |

`pkgcontents` is a instance of `PackageContents`, which is just an enumeration of the package's contents.

Here's the Rust definition:

<details>
    <summary>Rust definition</summary>

```rust
// PackageContents
/// Represents the contents of a package
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageContents {
    pub folders: Option<Vec<PackageFolder>>,
    pub files: Option<Vec<PackageFile>>,
    pub links: Option<Vec<PackageLink>>,
}

// PackageFolder
/// Represents a folder a package creates, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFolder {
    pub name: String,
    pub mtime: usize,
    pub installpath: String,
    pub meta: FileMetadata,
}

// PackageFile
/// Represents a file inside a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageFile {
    pub name: String,
    pub sha256: String,
    pub meta: FileMetadata,
    pub mtime: usize,
    pub installpath: String,
}

// PackageLink
/// Represents a symbolic link that should be created by a package, and its metadata
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageLink {
    pub file: String,
    pub mtime: usize,
    pub target: String,
}

// FileMetadata
/// Represents metadata on a file or folder
//
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileMetadata {
    pub owner: usize,
    pub group: usize,
    pub permissions: usize,
}
```
</details>

Here's the table for `PackageContents`:

| Field   | Optional | Type               | Description                                                             | Example   |
|---------|----------|--------------------|-------------------------------------------------------------------------|-----------|
| folders | yes      | Vec<PackageFolder> | A list of folders, if any, that are present inside this package.        | See below |
| files   | yes      | Vec<PackageFile>   | A list of files, if any, that are present inside this package.          | See below |
| links   | yes      | Vec<PackageLink>   | A list of symbolic links, if any, that are present inside this package. | See below |

The table for `PackageFolder`:

| Field       | Optional | Type         | Description                                          | Example            |
|-------------|----------|--------------|------------------------------------------------------|--------------------|
| name        | no       | String       | The name of the folder insize the package.           | "/opt/helloworld"  |
| mtime       | no       | usize        | The last modified time of the folder.                | 0                  |
| installpath | no       | String       | The path to install the folder to on the filesystem. | "/opt/helloworld"  |
| meta        | no       | FileMetadata | The file permissions and metadata                    | See below          |
> **Note:** Due to an intentional design decision while creating mangrove, while it is possible for `name` and `installpath` to be different, this constitutes an invalid package entry, and it will either error or be ignored.

`PackageFile`:

| Field       | Optional | Type         | Description                                          | Example                                                            |
|-------------|----------|--------------|------------------------------------------------------|--------------------------------------------------------------------|
| name        | no       | String       | The name of the folder insize the package.           | "/opt/helloworld/hi"                                               |
| sha256      | no       | String       | The sha256 hash of the file after decompression.     | "b2bbddeabbceef232628ada1603ba4dda1d29c73d360cfa0a608baa5a70a7152" |
| meta        | no       | FileMetadata | The file permissions and metadata                    | See below                                                          |
| mtime       | no       | usize        | The last modified time of the folder.                | 0                                                                  |
| installpath | no       | String       | The path to install the folder to on the filesystem. | "/opt/helloworld/hi"                                               |
> **Note:** Due to an intentional design decision while creating mangrove, while it is possible for `name` and `installpath` to be different, this constitutes an invalid package entry, and it will either error or be ignored.

`PackageLink`:

| Field  | Optional | Type    | Description                                              | Example           |
|--------|----------|---------|----------------------------------------------------------|-------------------|
| file   | no       | String  | The source file of the symbolic link                     | "/etc/fileA"      |
| mtime  | no       | usize   | The modification time of the **symlink**, not its target | 0                 |
| target | no       | String  | The target file of the symbolic link                     | "/etc/lnktofileA" |

Finally, `FileMetadata`:


| Field       | Optional | Type    | Description                     | Example |
|-------------|----------|---------|---------------------------------|---------|
| owner       | no       | usize   | The owner's UID                 | 1000    |
| group       | no       | usize   | The group's GID                 | 1000    |
| permissions | no       | usize   | The file permissions, numerical | 755     |

These structures are all serialized using [MessagePack](https://messagepack.org), and the result is saved to the pkginfo file.

<details>
    <summary>Why MessagePack?</summary>

To demonstrate why MessagePack was picked for this, here is the test_package@v1 serialized into json instead:
```json
[
  "test",
  "0.0.1",
  "A test package, used in Mangrove unit tests",
  "This is a longer package description for test, which is a test package uesd in mangrove unit tests.",
  "amd64",
  "https://mgve.cc",
  "GNU-GPL-3-or-later",
  [
    "thisisgroup1",
    "thisisgroup2"
  ],
  [
    [
      "test-data",
      "*"
    ],
    [
      "test-data-2",
      "^0.0.0"
    ]
  ],
  [
    "test-opt: for doing something else"
  ],
  [
    [
      "other-package",
      "*"
    ]
  ],
  [
    [
      "conflicting-package",
      "*"
    ]
  ],
  [
    [
      "old-package",
      "*"
    ]
  ],
  234234324,
  [
    [
      [
        "/hello_world",
        0,
        "/hello_world",
        [
          0,
          0,
          644
        ]
      ],
      [
        "/usr",
        0,
        "/usr",
        [
          0,
          0,
          644
        ]
      ],
      [
        "/usr/bin",
        0,
        "/usr/bin",
        [
          0,
          0,
          644
        ]
      ]
    ],
    [
      [
        "/hello_world/helloworld",
        "cb0659425446bd79e7699e858041748deaae8423f63e6feaf907bfbb9345a32b",
        [
          0,
          0,
          644
        ],
        0,
        "/hello_world/helloworld"
      ]
    ],
    [
      [
        "/hello_world/helloworld",
        0,
        "/usr/bin/helloworld"
      ]
    ]
  ]
]
```
This comes in at just over 1364 bytes.

On the other hand, here is the messagepack representation:
```hexdump
a49f 6574 7473 30a5 302e 312e 2bd9 2041
6574 7473 7020 6361 616b 6567 202c 7375
6465 6920 206e 614d 676e 6f72 6576 7520
696e 2074 6574 7473 d973 5463 6968 2073
7369 6120 6c20 6e6f 6567 2072 6170 6b63
6761 2065 6564 6373 6972 7470 6f69 206e
6f66 2072 6574 7473 202c 6877 6369 2068
7369 6120 7420 7365 2074 6170 6b63 6761
2065 6575 6473 6920 206e 616d 676e 6f72
6576 7520 696e 2074 6574 7473 2e73 61a5
646d 3436 68af 7474 7370 2f3a 6d2f 7667
2e65 6363 47b2 554e 472d 4c50 332d 6f2d
2d72 616c 6574 9272 74ac 6968 6973 6773
6f72 7075 ac31 6874 7369 7369 7267 756f
3270 9292 74a9 7365 2d74 6164 6174 2aa1
ab92 6574 7473 642d 7461 2d61 a632 305e
302e 302e d991 7422 7365 2d74 706f 3a74
6620 726f 6420 696f 676e 7320 6d6f 7465
6968 676e 6520 736c 9165 ad92 746f 6568
2d72 6170 6b63 6761 a165 912a b392 6f63
666e 696c 7463 6e69 2d67 6170 6b63 6761
a165 912a ab92 6c6f 2d64 6170 6b63 6761
a165 ce2a f60d d421 9393 ac94 682f 6c65
6f6c 775f 726f 646c ac00 682f 6c65 6f6c
775f 726f 646c 0093 cd00 8402 a494 752f
7273 a400 752f 7273 0093 cd00 8402 a894
752f 7273 622f 6e69 a800 752f 7273 622f
6e69 0093 cd00 8402 9591 2fb7 6568 6c6c
5f6f 6f77 6c72 2f64 6568 6c6c 776f 726f
646c 40d9 6263 3630 3935 3234 3435 3634
6462 3937 3765 3936 6539 3538 3038 3134
3437 6438 6165 6561 3438 3332 3666 6533
6636 6165 3966 3730 6662 6262 3339 3534
3361 6232 0093 cd00 8402 b700 682f 6c65
6f6c 775f 726f 646c 682f 6c65 6f6c 6f77
6c72 9164 b793 682f 6c65 6f6c 775f 726f
646c 682f 6c65 6f6c 6f77 6c72 0064 2fb3
7375 2f72 6962 2f6e 6568 6c6c 776f 726f
646c
```
which comes out to just over 600 bytes for a 57% reduction. Pretty cool!

</details>