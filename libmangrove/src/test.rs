//! # libmangrove tests
//! This module contains the libmangrove test suite. It does nothing useful otherwise.

#[cfg(test)]
mod tests {

    use crate::{
        aes::{AES128Cipher, AES192Cipher, AES256Cipher},
        crypt::{decrypt_package, encrypt_package, PrivateKey},
        file::FileOps,
        pkg::{
            save_package, FileMetadata, Package, PackageContents, PackageFile, PackageFolder,
            PackageLink, PkgSpec,
        },
        platform::Architecture,
        repo::RepoPackage,
    };
    use version::{BuildMetadata, Prerelease, Version, VersionReq};

    fn get_test_package() -> Package {
        // test_package@v1, if this changes the below byte repr also has to be updated!
        let pkg: Package = Package {
            pkgname: "test".to_string(),
            pkgver: Version { major: 0, minor: 0, patch: 1, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY},
            shortdesc: "A test package, used in Mangrove unit tests".to_string(),
            longdesc: Some("This is a longer package description for test, which is a test package uesd in mangrove unit tests.".to_string()),
            arch: Architecture::Amd64,
            url: Some("https://mgve.cc".to_string()),
            license: Some("GNU-GPL-3-or-later".to_string()),
            groups: Some(vec!["thisisgroup1".to_string(), "thisisgroup2".to_string()]),
            depends: Some(vec![PkgSpec {
                pkgname: "test-data".to_string(),
                version: VersionReq { comparators: vec![] }
            }, PkgSpec {
                pkgname: "test-data-2".to_string(),
                version: VersionReq::parse("0.0.0").unwrap()
            }]),
            optdepends: Some(vec!["test-opt: for doing something else".to_string()]),
            provides: Some(vec![
                PkgSpec {
                    pkgname: "other-package".to_string(),
                    version: VersionReq { comparators: vec![] }
                }
            ]),
            conflicts: Some(vec![
                PkgSpec {
                    pkgname: "conflicting-package".to_string(),
                    version: VersionReq { comparators: vec![] }
                }
            ]),
            replaces: Some(vec![
                PkgSpec {
                    pkgname: "old-package".to_string(),
                    version: VersionReq { comparators: vec![] }
                }
            ]),
            installed_size: 234234324,
            pkgcontents: PackageContents {
                folders: Some(vec![
                    PackageFolder {
                        name: "/hello_world".to_string(),
                        mtime: 0,
                        installpath: "/hello_world".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644
                        },
                    },
                    PackageFolder {
                        name: "/usr".to_string(),
                        mtime: 0,
                        installpath: "/usr".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644
                        },
                    },
                    PackageFolder {
                        name: "/usr/bin".to_string(),
                        mtime: 0,
                        installpath: "/usr/bin".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644
                        },
                    }
                ]),
                files: Some(vec![
                    PackageFile {
                        name: "/hello_world/helloworld".to_string(),
                        sha256: "cb0659425446bd79e7699e858041748deaae8423f63e6feaf907bfbb9345a32b".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644
                        },
                        mtime: 0,
                        installpath: "/hello_world/helloworld".to_string()
                    }
                ]),
                links: Some(vec![
                    PackageLink {
                        file: "/hello_world/helloworld".to_string(),
                        mtime: 0,
                        target: "/usr/bin/helloworld".to_string()
                    }
                ])
            }
        };
        pkg
    }

    fn get_test_package_bytes() -> Vec<u8> {
        // This is ugly but the easiest way of doing this
        // Byte data for test_package@v1
        vec![
            159, 164, 116, 101, 115, 116, 165, 48, 46, 48, 46, 49, 217, 43, 65, 32, 116, 101, 115,
            116, 32, 112, 97, 99, 107, 97, 103, 101, 44, 32, 117, 115, 101, 100, 32, 105, 110, 32,
            77, 97, 110, 103, 114, 111, 118, 101, 32, 117, 110, 105, 116, 32, 116, 101, 115, 116,
            115, 217, 99, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 108, 111, 110, 103, 101,
            114, 32, 112, 97, 99, 107, 97, 103, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116,
            105, 111, 110, 32, 102, 111, 114, 32, 116, 101, 115, 116, 44, 32, 119, 104, 105, 99,
            104, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 112, 97, 99, 107, 97, 103, 101,
            32, 117, 101, 115, 100, 32, 105, 110, 32, 109, 97, 110, 103, 114, 111, 118, 101, 32,
            117, 110, 105, 116, 32, 116, 101, 115, 116, 115, 46, 165, 65, 109, 100, 54, 52, 175,
            104, 116, 116, 112, 115, 58, 47, 47, 109, 103, 118, 101, 46, 99, 99, 178, 71, 78, 85,
            45, 71, 80, 76, 45, 51, 45, 111, 114, 45, 108, 97, 116, 101, 114, 146, 172, 116, 104,
            105, 115, 105, 115, 103, 114, 111, 117, 112, 49, 172, 116, 104, 105, 115, 105, 115,
            103, 114, 111, 117, 112, 50, 146, 146, 169, 116, 101, 115, 116, 45, 100, 97, 116, 97,
            161, 42, 146, 171, 116, 101, 115, 116, 45, 100, 97, 116, 97, 45, 50, 166, 94, 48, 46,
            48, 46, 48, 145, 217, 34, 116, 101, 115, 116, 45, 111, 112, 116, 58, 32, 102, 111, 114,
            32, 100, 111, 105, 110, 103, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 101,
            108, 115, 101, 145, 146, 173, 111, 116, 104, 101, 114, 45, 112, 97, 99, 107, 97, 103,
            101, 161, 42, 145, 146, 179, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103, 45,
            112, 97, 99, 107, 97, 103, 101, 161, 42, 145, 146, 171, 111, 108, 100, 45, 112, 97, 99,
            107, 97, 103, 101, 161, 42, 206, 13, 246, 33, 212, 147, 147, 148, 172, 47, 104, 101,
            108, 108, 111, 95, 119, 111, 114, 108, 100, 0, 172, 47, 104, 101, 108, 108, 111, 95,
            119, 111, 114, 108, 100, 147, 0, 0, 205, 2, 132, 148, 164, 47, 117, 115, 114, 0, 164,
            47, 117, 115, 114, 147, 0, 0, 205, 2, 132, 148, 168, 47, 117, 115, 114, 47, 98, 105,
            110, 0, 168, 47, 117, 115, 114, 47, 98, 105, 110, 147, 0, 0, 205, 2, 132, 145, 149,
            183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108,
            111, 119, 111, 114, 108, 100, 217, 64, 99, 98, 48, 54, 53, 57, 52, 50, 53, 52, 52, 54,
            98, 100, 55, 57, 101, 55, 54, 57, 57, 101, 56, 53, 56, 48, 52, 49, 55, 52, 56, 100,
            101, 97, 97, 101, 56, 52, 50, 51, 102, 54, 51, 101, 54, 102, 101, 97, 102, 57, 48, 55,
            98, 102, 98, 98, 57, 51, 52, 53, 97, 51, 50, 98, 147, 0, 0, 205, 2, 132, 0, 183, 47,
            104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119,
            111, 114, 108, 100, 145, 147, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108,
            100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 0, 179, 47, 117, 115, 114,
            47, 98, 105, 110, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100,
        ]
    }

    fn get_test_repopackage() -> RepoPackage {
        let repopkg: RepoPackage = RepoPackage {
            package_data: get_test_package(),
            avaliable_versions: vec![
                Version::parse("0.0.1").expect("Failure while parsing version")
            ],
        };
        repopkg
    }

    fn get_test_repopackage_bytes() -> Vec<u8> {
        vec![
            146, 159, 164, 116, 101, 115, 116, 165, 48, 46, 48, 46, 49, 217, 43, 65, 32, 116, 101,
            115, 116, 32, 112, 97, 99, 107, 97, 103, 101, 44, 32, 117, 115, 101, 100, 32, 105, 110,
            32, 77, 97, 110, 103, 114, 111, 118, 101, 32, 117, 110, 105, 116, 32, 116, 101, 115,
            116, 115, 217, 99, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 108, 111, 110, 103,
            101, 114, 32, 112, 97, 99, 107, 97, 103, 101, 32, 100, 101, 115, 99, 114, 105, 112,
            116, 105, 111, 110, 32, 102, 111, 114, 32, 116, 101, 115, 116, 44, 32, 119, 104, 105,
            99, 104, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 112, 97, 99, 107, 97, 103,
            101, 32, 117, 101, 115, 100, 32, 105, 110, 32, 109, 97, 110, 103, 114, 111, 118, 101,
            32, 117, 110, 105, 116, 32, 116, 101, 115, 116, 115, 46, 165, 65, 109, 100, 54, 52,
            175, 104, 116, 116, 112, 115, 58, 47, 47, 109, 103, 118, 101, 46, 99, 99, 178, 71, 78,
            85, 45, 71, 80, 76, 45, 51, 45, 111, 114, 45, 108, 97, 116, 101, 114, 146, 172, 116,
            104, 105, 115, 105, 115, 103, 114, 111, 117, 112, 49, 172, 116, 104, 105, 115, 105,
            115, 103, 114, 111, 117, 112, 50, 146, 146, 169, 116, 101, 115, 116, 45, 100, 97, 116,
            97, 161, 42, 146, 171, 116, 101, 115, 116, 45, 100, 97, 116, 97, 45, 50, 166, 94, 48,
            46, 48, 46, 48, 145, 217, 34, 116, 101, 115, 116, 45, 111, 112, 116, 58, 32, 102, 111,
            114, 32, 100, 111, 105, 110, 103, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32,
            101, 108, 115, 101, 145, 146, 173, 111, 116, 104, 101, 114, 45, 112, 97, 99, 107, 97,
            103, 101, 161, 42, 145, 146, 179, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103,
            45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 145, 146, 171, 111, 108, 100, 45, 112, 97,
            99, 107, 97, 103, 101, 161, 42, 206, 13, 246, 33, 212, 147, 147, 148, 172, 47, 104,
            101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 0, 172, 47, 104, 101, 108, 108, 111,
            95, 119, 111, 114, 108, 100, 147, 0, 0, 205, 2, 132, 148, 164, 47, 117, 115, 114, 0,
            164, 47, 117, 115, 114, 147, 0, 0, 205, 2, 132, 148, 168, 47, 117, 115, 114, 47, 98,
            105, 110, 0, 168, 47, 117, 115, 114, 47, 98, 105, 110, 147, 0, 0, 205, 2, 132, 145,
            149, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108,
            108, 111, 119, 111, 114, 108, 100, 217, 64, 99, 98, 48, 54, 53, 57, 52, 50, 53, 52, 52,
            54, 98, 100, 55, 57, 101, 55, 54, 57, 57, 101, 56, 53, 56, 48, 52, 49, 55, 52, 56, 100,
            101, 97, 97, 101, 56, 52, 50, 51, 102, 54, 51, 101, 54, 102, 101, 97, 102, 57, 48, 55,
            98, 102, 98, 98, 57, 51, 52, 53, 97, 51, 50, 98, 147, 0, 0, 205, 2, 132, 0, 183, 47,
            104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119,
            111, 114, 108, 100, 145, 147, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108,
            100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 0, 179, 47, 117, 115, 114,
            47, 98, 105, 110, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 145, 165, 48,
            46, 48, 46, 49,
        ]
    }

    #[test]
    fn package_serialization() {
        let pkg = get_test_package();
        let serialized: Vec<u8> = rmp_serde::to_vec(&pkg).unwrap();
        let expect: Vec<u8> = get_test_package_bytes();
        assert_eq!(serialized, expect);
    }

    #[test]
    fn package_deserialization() {
        let serialized: Vec<u8> = get_test_package_bytes();
        let deserialized: Package = rmp_serde::from_slice(&serialized[..]).unwrap();
        let expect: Package = get_test_package();
        assert_eq!(deserialized, expect);
    }

    #[test]
    fn package_save_to_file_does_not_exist() {
        // Get a test package
        let package: Package = get_test_package();
        // Try to write
        assert!(Package::to_file(&package, "/path/nonexistent-file/".to_string()).is_err());
    }

    #[test]
    fn package_save_to_file_without_permissions() {
        let package: Package = get_test_package();
        assert!(Package::to_file(&package, "/root/cant-write-here/".to_string()).is_err());
    }

    #[test]
    fn repo_package_serialization() {
        // Get a test package
        let repopkg: RepoPackage = get_test_repopackage();
        let serialized: Vec<u8> =
            rmp_serde::to_vec(&repopkg).expect("Failed to serialize package data");
        assert_eq!(serialized, get_test_repopackage_bytes());
    }

    #[test]
    fn package_saving() {
        match save_package(
            get_test_package(),
            "/home/core/prj/personal/mangrove/test/test-package".to_string(),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn mcrypt_aes128() {
        let key = [42u8; 16];
        let mut cipher = AES128Cipher::new(key);
        let data = "abcdef".to_string().into_bytes();
        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(data, decrypted);
    }

    #[test]
    fn mcrypt_aes192() {
        let key = [42u8; 24];
        let mut cipher = AES192Cipher::new(key);
        let data = "abcdefghijklmnopqrstuvwxyz".to_string().into_bytes();
        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(data, decrypted);
    }

    #[test]
    fn mcrypt_aes256() {
        let key = [42u8; 32];
        let mut cipher = AES256Cipher::new(key);
        let data = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789)!@#$%^&*("
            .to_string()
            .into_bytes();
        let encrypted = cipher.encrypt(&data);
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(data, decrypted);
    }

    #[test]
    fn mcrypt_pkg_encryption() {
        let pkg = get_test_package_bytes(); // Get a test package in bytes
        let sk = PrivateKey::generate("testkey".to_string());
        let pk = sk.derive();
        let encrypted = match encrypt_package(&sk, &pkg) {
            Ok(e) => e,
            Err(err) => panic!("{}", err),
        };
        let decrypted = match decrypt_package(&pk, encrypted) {
            Ok(d) => d,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(pkg, decrypted);
    }
}
