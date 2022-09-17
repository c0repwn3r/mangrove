//! # libmangrove tests
//! This module contains the libmangrove test suite. It does nothing useful otherwise.

mod libmangrove_tests_common {
    use version::{BuildMetadata, Prerelease, Version, VersionReq};
    use crate::crypt::{PrivateKey, PublicKey};
    use crate::pkg::{FileMetadata, Package, PackageContents, PackageFile, PackageFolder, PackageLink, PkgSpec};
    use crate::platform::Architecture;
    use crate::repo::RepoPackage;

    #[allow(unused)]
    pub fn get_test_package() -> Package {
        // test_package@v1, if this changes the below byte repr also has to be updated!
        let pkg: Package = Package {
            pkgname: "test".to_string(),
            pkgver: Version { major: 0, minor: 0, patch: 1, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY },
            shortdesc: "A test package, used in Mangrove unit tests".to_string(),
            longdesc: Some("This is a longer package description for test, which is a test package uesd in mangrove unit tests.".to_string()),
            arch: Architecture::amd64,
            url: Some("https://mgve.cc".to_string()),
            license: Some("GNU-GPL-3-or-later".to_string()),
            groups: Some(vec!["thisisgroup1".to_string(), "thisisgroup2".to_string()]),
            depends: Some(vec![PkgSpec {
                pkgname: "test-data".to_string(),
                version: VersionReq { comparators: vec![] },
            }, PkgSpec {
                pkgname: "test-data-2".to_string(),
                version: VersionReq::parse("0.0.0").unwrap(),
            }]),
            optdepends: Some(vec!["test-opt: for doing something else".to_string()]),
            provides: Some(vec![
                PkgSpec {
                    pkgname: "other-package".to_string(),
                    version: VersionReq { comparators: vec![] },
                }
            ]),
            conflicts: Some(vec![
                PkgSpec {
                    pkgname: "conflicting-package".to_string(),
                    version: VersionReq { comparators: vec![] },
                }
            ]),
            replaces: Some(vec![
                PkgSpec {
                    pkgname: "old-package".to_string(),
                    version: VersionReq { comparators: vec![] },
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
                            permissions: 644,
                        },
                    },
                    PackageFolder {
                        name: "/usr".to_string(),
                        mtime: 0,
                        installpath: "/usr".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644,
                        },
                    },
                    PackageFolder {
                        name: "/usr/bin".to_string(),
                        mtime: 0,
                        installpath: "/usr/bin".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644,
                        },
                    },
                ]),
                files: Some(vec![
                    PackageFile {
                        name: "/hello_world/helloworld".to_string(),
                        sha256: "cb0659425446bd79e7699e858041748deaae8423f63e6feaf907bfbb9345a32b".to_string(),
                        meta: FileMetadata {
                            owner: 0,
                            group: 0,
                            permissions: 644,
                        },
                        mtime: 0,
                        installpath: "/hello_world/helloworld".to_string(),
                    }
                ]),
                links: Some(vec![
                    PackageLink {
                        file: "/hello_world/helloworld".to_string(),
                        mtime: 0,
                        target: "/usr/bin/helloworld".to_string(),
                    }
                ]),
            },
        };
        pkg
    }

    #[allow(unused)]
    pub fn get_test_package_bytes() -> Vec<u8> {
        // This is ugly but the easiest way of doing this
        // Byte data for test_package@v1
        vec![159, 164, 116, 101, 115, 116, 165, 48, 46, 48, 46, 49, 217, 43, 65, 32, 116, 101, 115, 116, 32, 112, 97, 99, 107, 97, 103, 101, 44, 32, 117, 115, 101, 100, 32, 105, 110, 32, 77, 97, 110, 103, 114, 111, 118, 101, 32, 117, 110, 105, 116, 32, 116, 101, 115, 116, 115, 217, 99, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 108, 111, 110, 103, 101, 114, 32, 112, 97, 99, 107, 97, 103, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 102, 111, 114, 32, 116, 101, 115, 116, 44, 32, 119, 104, 105, 99, 104, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 112, 97, 99, 107, 97, 103, 101, 32, 117, 101, 115, 100, 32, 105, 110, 32, 109, 97, 110, 103, 114, 111, 118, 101, 32, 117, 110, 105, 116, 32, 116, 101, 115, 116, 115, 46, 165, 97, 109, 100, 54, 52, 175, 104, 116, 116, 112, 115, 58, 47, 47, 109, 103, 118, 101, 46, 99, 99, 178, 71, 78, 85, 45, 71, 80, 76, 45, 51, 45, 111, 114, 45, 108, 97, 116, 101, 114, 146, 172, 116, 104, 105, 115, 105, 115, 103, 114, 111, 117, 112, 49, 172, 116, 104, 105, 115, 105, 115, 103, 114, 111, 117, 112, 50, 146, 146, 169, 116, 101, 115, 116, 45, 100, 97, 116, 97, 161, 42, 146, 171, 116, 101, 115, 116, 45, 100, 97, 116, 97, 45, 50, 166, 94, 48, 46, 48, 46, 48, 145, 217, 34, 116, 101, 115, 116, 45, 111, 112, 116, 58, 32, 102, 111, 114, 32, 100, 111, 105, 110, 103, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 101, 108, 115, 101, 145, 146, 173, 111, 116, 104, 101, 114, 45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 145, 146, 179, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103, 45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 145, 146, 171, 111, 108, 100, 45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 206, 13, 246, 33, 212, 147, 147, 148, 172, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 0, 172, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 147, 0, 0, 205, 2, 132, 148, 164, 47, 117, 115, 114, 0, 164, 47, 117, 115, 114, 147, 0, 0, 205, 2, 132, 148, 168, 47, 117, 115, 114, 47, 98, 105, 110, 0, 168, 47, 117, 115, 114, 47, 98, 105, 110, 147, 0, 0, 205, 2, 132, 145, 149, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 217, 64, 99, 98, 48, 54, 53, 57, 52, 50, 53, 52, 52, 54, 98, 100, 55, 57, 101, 55, 54, 57, 57, 101, 56, 53, 56, 48, 52, 49, 55, 52, 56, 100, 101, 97, 97, 101, 56, 52, 50, 51, 102, 54, 51, 101, 54, 102, 101, 97, 102, 57, 48, 55, 98, 102, 98, 98, 57, 51, 52, 53, 97, 51, 50, 98, 147, 0, 0, 205, 2, 132, 0, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 145, 147, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 0, 179, 47, 117, 115, 114, 47, 98, 105, 110, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100]
    }

    #[allow(unused)]
    pub fn get_test_repopackage() -> RepoPackage {
        let repopkg: RepoPackage = RepoPackage {
            package_data: get_test_package(),
            avaliable_versions: vec![
                Version::parse("0.0.1").expect("Failure while parsing version")
            ],
        };
        repopkg
    }

    #[allow(unused)]
    pub fn get_test_repopackage_bytes() -> Vec<u8> {
        vec![146, 159, 164, 116, 101, 115, 116, 165, 48, 46, 48, 46, 49, 217, 43, 65, 32, 116, 101, 115, 116, 32, 112, 97, 99, 107, 97, 103, 101, 44, 32, 117, 115, 101, 100, 32, 105, 110, 32, 77, 97, 110, 103, 114, 111, 118, 101, 32, 117, 110, 105, 116, 32, 116, 101, 115, 116, 115, 217, 99, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 108, 111, 110, 103, 101, 114, 32, 112, 97, 99, 107, 97, 103, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 102, 111, 114, 32, 116, 101, 115, 116, 44, 32, 119, 104, 105, 99, 104, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 112, 97, 99, 107, 97, 103, 101, 32, 117, 101, 115, 100, 32, 105, 110, 32, 109, 97, 110, 103, 114, 111, 118, 101, 32, 117, 110, 105, 116, 32, 116, 101, 115, 116, 115, 46, 165, 97, 109, 100, 54, 52, 175, 104, 116, 116, 112, 115, 58, 47, 47, 109, 103, 118, 101, 46, 99, 99, 178, 71, 78, 85, 45, 71, 80, 76, 45, 51, 45, 111, 114, 45, 108, 97, 116, 101, 114, 146, 172, 116, 104, 105, 115, 105, 115, 103, 114, 111, 117, 112, 49, 172, 116, 104, 105, 115, 105, 115, 103, 114, 111, 117, 112, 50, 146, 146, 169, 116, 101, 115, 116, 45, 100, 97, 116, 97, 161, 42, 146, 171, 116, 101, 115, 116, 45, 100, 97, 116, 97, 45, 50, 166, 94, 48, 46, 48, 46, 48, 145, 217, 34, 116, 101, 115, 116, 45, 111, 112, 116, 58, 32, 102, 111, 114, 32, 100, 111, 105, 110, 103, 32, 115, 111, 109, 101, 116, 104, 105, 110, 103, 32, 101, 108, 115, 101, 145, 146, 173, 111, 116, 104, 101, 114, 45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 145, 146, 179, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103, 45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 145, 146, 171, 111, 108, 100, 45, 112, 97, 99, 107, 97, 103, 101, 161, 42, 206, 13, 246, 33, 212, 147, 147, 148, 172, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 0, 172, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 147, 0, 0, 205, 2, 132, 148, 164, 47, 117, 115, 114, 0, 164, 47, 117, 115, 114, 147, 0, 0, 205, 2, 132, 148, 168, 47, 117, 115, 114, 47, 98, 105, 110, 0, 168, 47, 117, 115, 114, 47, 98, 105, 110, 147, 0, 0, 205, 2, 132, 145, 149, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 217, 64, 99, 98, 48, 54, 53, 57, 52, 50, 53, 52, 52, 54, 98, 100, 55, 57, 101, 55, 54, 57, 57, 101, 56, 53, 56, 48, 52, 49, 55, 52, 56, 100, 101, 97, 97, 101, 56, 52, 50, 51, 102, 54, 51, 101, 54, 102, 101, 97, 102, 57, 48, 55, 98, 102, 98, 98, 57, 51, 52, 53, 97, 51, 50, 98, 147, 0, 0, 205, 2, 132, 0, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 145, 147, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 0, 179, 47, 117, 115, 114, 47, 98, 105, 110, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 145, 165, 48, 46, 48, 46, 49]
    }

    #[allow(unused)]
    pub fn get_test_privkey() -> PrivateKey {
        PrivateKey::from_anonymous("AWxDWGKXZZOndWlvY5gvsbLzeRJEFpueNUoR/VCDKXMtBoeIyZoHATvrJWgu5vG2XlEqAbZuUGtCRERaa2aBPw==".to_string()).unwrap()
    }

    #[allow(unused)]
    pub fn get_test_pubkey() -> PublicKey {
        PublicKey::from_anonymous("LQaHiMmaBwE76yVoLubxtl5RKgG2blBrQkREWmtmgT8=".to_string()).unwrap()
    }
}

#[cfg(test)]
mod libmangrove_pkg_tests {
    use std::{env, fs};
    use crate::crypt::{is_signed_package};
    use crate::file::FileOps;
    use crate::pkg::{get_pkg_filename, Package, save_package, save_package_signed};
    use crate::test::libmangrove_tests_common::{get_test_package, get_test_package_bytes, get_test_privkey};
    use serial_test::serial;

    #[test]
    fn package_serialization() {
        let pkg = get_test_package();
        let serialized: Vec<u8> = rmp_serde::to_vec(&pkg).unwrap();
        println!("{:?}", serialized);
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
        assert!(Package::as_file(&package, "/path/nonexistent-file/".to_string()).is_err());
    }

    #[test]
    #[should_panic]
    fn package_save_to_file_without_permissions() {
        let package: Package = get_test_package();
        Package::as_file(&package, "/root/cant-write-here/".to_string()).unwrap();
    }

    #[test]
    #[serial] // If run concurrently with package_saving_signed, will fail
    fn package_saving() {
        println!("{:?}", env::current_dir().unwrap());
        match save_package(
            get_test_package(),
            format!("{}/../test/test-package", env::current_dir().unwrap().to_str().unwrap()),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    #[serial] // If run concurrently with package_saving, will fail
    fn package_saving_signed() {
        match save_package_signed(
            get_test_package(),
            format!("{}/../test/test-package-signed", env::current_dir().unwrap().to_str().unwrap()),
            get_test_privkey(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    #[serial]
    fn package_validating_signed() {
        match save_package_signed(
            get_test_package(),
            format!("{}/../test/test-package", env::current_dir().unwrap().to_str().unwrap()),
            get_test_privkey(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };
        let fname = format!("../test/test-package/{}", get_pkg_filename(&get_test_package()));
        let signed_package_data = fs::read(fname).unwrap();
        assert!(is_signed_package(signed_package_data));
    }

    #[test]
    fn package_fileops_load() {
        match Package::as_file(&get_test_package(), "../test/test_pkginfo".parse().unwrap()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        let newpkg = match Package::from_file("../test/test_pkginfo".parse().unwrap()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(newpkg, get_test_package());
    }

    #[test]
    #[should_panic]
    fn package_fileops_load_permission_denied() {
        Package::from_file("/root/permissiondenied".parse().unwrap()).unwrap();
    }

    #[test]
    #[should_panic]
    fn package_fileops_load_not_a_real_package() {
        Package::from_file("./test/test-package/hello_world/helloworld".parse().unwrap()).unwrap();
    }
}

#[cfg(test)]
mod libmangrove_repopackage_tests {
    use crate::repo::RepoPackage;
    use crate::test::libmangrove_tests_common::{get_test_repopackage, get_test_repopackage_bytes};

    #[test]
    fn repo_package_serialization() {
        // Get a test package
        let repopkg: RepoPackage = get_test_repopackage();
        let serialized: Vec<u8> =
            rmp_serde::to_vec(&repopkg).expect("Failed to serialize package data");
        assert_eq!(serialized, get_test_repopackage_bytes());
    }

    #[test]
    fn repo_package_deserialization() {

        // Get a test package
        let repopkg_bytes = get_test_repopackage_bytes();
        let deserialized: RepoPackage = rmp_serde::from_slice(&repopkg_bytes[..]).expect("Failed to deserialize package data");
        assert_eq!(deserialized, get_test_repopackage());
    }
}

#[cfg(test)]
mod libmangrove_mcrypt_tests {
    use crate::aes::{AES128Cipher, AES192Cipher, AES256Cipher};
    use crate::crypt::{debug_dump_package, decrypt_package, encrypt_package, PrivateKey};
    use crate::test::libmangrove_tests_common::{get_test_package_bytes, get_test_privkey, get_test_pubkey};

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
        let sk = get_test_privkey();
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

    #[test]
    fn mcrypt_pkg_dump() {
        let pkg = get_test_package_bytes();
        let sk = PrivateKey::generate("test_key".to_string());
        let pk = sk.derive();
        let encrypted = match encrypt_package(&sk, &pkg) {
            Ok(e) => e,
            Err(err) => panic!("{}", err)
        };
        println!("{}", debug_dump_package(encrypted.clone(), Some(&pk)));
        println!("{}", debug_dump_package(encrypted.clone(), None));
    }

    #[test]
    fn mcrypt_privkey_anonymous() {
        // this also tests from_anonymous, as it is used to derive the private key (see common)
        let key = get_test_privkey();
        assert_eq!(key.to_anonymous(), "AWxDWGKXZZOndWlvY5gvsbLzeRJEFpueNUoR/VCDKXMtBoeIyZoHATvrJWgu5vG2XlEqAbZuUGtCRERaa2aBPw==".to_string());
    }

    #[test]
    fn mcrypt_pubkey_anonymous() {
        // this also tests from_anonymous, as it is used to derive the public key (see common)
        let key = get_test_pubkey();
        assert_eq!(key.to_anonymous(), "LQaHiMmaBwE76yVoLubxtl5RKgG2blBrQkREWmtmgT8=".to_string());
    }
}

#[cfg(test)]
mod libmangrove_lockfile_tests {
    use crate::lock::{lock_packages, lock_repository, lock_trustcache};
    use serial_test::serial;

    #[test]
    #[serial]
    fn repository_locking() {
        let lock = lock_repository(true).unwrap();
        lock.release().unwrap();
    }

    #[test]
    #[serial]
    fn repository_locking_already_locked() {
        let lock = lock_repository(true).unwrap();
        assert!(lock_repository(true).is_err());
        lock.release().unwrap();
    }

    #[test]
    #[serial]
    fn trustcache_locking() {
        let lock = lock_trustcache(true).unwrap();
        lock.release().unwrap();
    }

    #[test]
    #[serial]
    fn trustcache_locking_already_locked() {
        let lock = lock_trustcache(true).unwrap();
        assert!(lock_trustcache(true).is_err());
        lock.release().unwrap();
    }

    #[test]
    #[serial]
    fn package_locking() {
        let lock = lock_packages(true).unwrap();
        lock.release().unwrap();
    }

    #[test]
    #[serial]
    fn package_locking_already_locked() {
        let lock = lock_packages(true).unwrap();
        assert!(lock_packages(true).is_err());
        lock.release().unwrap();
    }
}

#[cfg(test)]
mod libmangrove_database_tests {
    use crate::trustcache::{trustcache_load, trustcache_save};
    use serial_test::serial;

    #[test]
    #[serial]
    fn trustcache() {
        let trustcache = trustcache_load(true).unwrap();
        trustcache_save(trustcache, true).unwrap();
    }
}