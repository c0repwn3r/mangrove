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

| Field     | Optional | Type    | Description                         | Example                                      |
|-----------|----------|---------|-------------------------------------|----------------------------------------------|
| pkgname   | no       | String  | The name of the package             | test                                         |
| pkgver    | no       | Version | The version of the package          | 0.1.1                                        |
| shortdesc | no       | String  | A short description of the package  | Says hello.                                  |
| longdesc  | yes      | String  | A longer description of the package | It says hello when you run the hello binary. |
| arch      | yes      |         |                                     |                                              |