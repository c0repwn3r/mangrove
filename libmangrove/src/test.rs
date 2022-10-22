//! # libmangrove tests
//! This module contains the libmangrove test suite. It does nothing useful otherwise.

#[cfg(test)]
mod libmangrove_tests_common {
    use url::Url;
    use version::{BuildMetadata, Prerelease, Version, VersionReq};

    use crate::crypt::{PrivateKey, PublicKey};
    use crate::pkg::{FileMetadata, Package, PackageContents, PackageFile, PackageFolder, PackageLink, PkgSpec};
    use crate::platform::Architecture;

    #[allow(unused)]
    pub fn logging() {
        simple_logger::init().unwrap();
    }

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
    pub fn get_test_nonsense_package_bytes() -> Vec<u8> {
        vec![159, 217, 35, 89, 93, 28, 209, 184, 239, 191, 189, 239, 191, 189, 239, 191, 189, 84, 94, 88, 239, 191, 189, 42, 239, 191, 189, 239, 191, 189, 239, 191, 189, 58, 51, 57, 17, 58, 171, 57, 57, 57, 46, 57, 57, 57, 46, 57, 57, 57, 186, 9, 69, 117, 239, 191, 189, 96, 239, 191, 189, 239, 191, 189, 239, 191, 189, 9, 239, 191, 189, 239, 191, 189, 239, 191, 189, 217, 230, 36, 239, 191, 189, 52, 25, 67, 79, 98, 239, 191, 189, 239, 191, 189, 239, 191, 189, 24, 239, 191, 189, 239, 191, 189, 0, 2, 20, 239, 191, 189, 111, 2, 88, 100, 57, 64, 42, 239, 191, 189, 239, 191, 189, 89, 239, 191, 189, 125, 239, 191, 189, 117, 239, 191, 189, 99, 3, 38, 110, 239, 191, 189, 239, 191, 189, 239, 191, 189, 117, 62, 46, 108, 44, 78, 239, 191, 189, 213, 159, 239, 191, 189, 45, 239, 191, 189, 73, 239, 191, 189, 239, 191, 189, 239, 191, 189, 239, 191, 189, 78, 239, 191, 189, 239, 191, 189, 60, 239, 191, 189, 108, 116, 239, 191, 189, 126, 239, 191, 189, 239, 191, 189, 7, 239, 191, 189, 62, 239, 191, 189, 78, 239, 191, 189, 239, 191, 189, 65, 82, 239, 191, 189, 10, 20, 239, 191, 189, 100, 239, 191, 189, 25, 239, 191, 189, 239, 191, 189, 239, 191, 189, 98, 239, 191, 189, 239, 191, 189, 239, 191, 189, 123, 48, 67, 239, 191, 189, 121, 9, 73, 200, 147, 18, 239, 191, 189, 98, 106, 239, 191, 189, 110, 239, 191, 189, 239, 191, 189, 79, 83, 239, 191, 189, 94, 43, 239, 191, 189, 239, 191, 189, 239, 191, 189, 51, 42, 239, 191, 189, 61, 239, 191, 189, 239, 191, 189, 22, 200, 142, 165, 97, 109, 100, 54, 52, 217, 45, 112, 57, 83, 100, 239, 191, 189, 26, 115, 239, 191, 189, 37, 239, 191, 189, 239, 191, 189, 72, 64, 74, 239, 191, 189, 232, 136, 141, 239, 191, 189, 45, 239, 191, 189, 239, 191, 189, 36, 239, 191, 189, 239, 191, 189, 217, 96, 25, 239, 191, 189, 45, 239, 191, 189, 99, 239, 191, 189, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 239, 191, 189, 239, 191, 189, 239, 191, 189, 102, 29, 14, 239, 191, 189, 239, 191, 189, 82, 87, 239, 191, 189, 42, 239, 191, 189, 85, 49, 70, 10, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 146, 217, 47, 52, 15, 79, 239, 191, 189, 239, 191, 189, 239, 191, 189, 75, 239, 191, 189, 66, 239, 191, 189, 63, 208, 135, 88, 239, 191, 189, 50, 239, 191, 189, 239, 191, 189, 2, 46, 239, 191, 189, 239, 191, 189, 53, 55, 239, 191, 189, 217, 43, 97, 205, 162, 239, 191, 189, 76, 49, 239, 191, 189, 20, 51, 239, 191, 189, 239, 191, 189, 65, 239, 191, 189, 239, 191, 189, 28, 79, 63, 125, 239, 191, 189, 10, 107, 33, 239, 191, 189, 104, 239, 191, 189, 146, 146, 217, 47, 239, 191, 189, 63, 61, 239, 191, 189, 239, 191, 189, 239, 191, 189, 239, 191, 189, 107, 239, 191, 189, 112, 101, 113, 239, 191, 189, 103, 7, 56, 47, 239, 191, 189, 239, 191, 189, 120, 239, 191, 189, 127, 239, 191, 189, 56, 115, 161, 42, 146, 217, 43, 239, 191, 189, 2, 53, 79, 66, 58, 63, 10, 63, 239, 191, 189, 239, 191, 189, 239, 191, 189, 29, 124, 69, 239, 191, 189, 239, 191, 189, 124, 239, 191, 189, 75, 239, 191, 189, 127, 239, 191, 189, 125, 7, 217, 35, 94, 49, 53, 51, 52, 51, 53, 52, 51, 53, 46, 50, 51, 52, 51, 52, 53, 50, 51, 53, 54, 46, 51, 50, 53, 52, 50, 52, 53, 52, 51, 53, 52, 51, 53, 145, 217, 52, 239, 191, 189, 239, 191, 189, 116, 126, 96, 239, 191, 189, 239, 191, 189, 239, 191, 189, 94, 239, 191, 189, 239, 191, 189, 44, 239, 191, 189, 239, 191, 189, 58, 71, 239, 191, 189, 239, 191, 189, 60, 239, 191, 189, 239, 191, 189, 239, 191, 189, 101, 126, 145, 146, 217, 44, 239, 191, 189, 89, 239, 191, 189, 79, 239, 191, 189, 103, 199, 190, 239, 191, 189, 239, 191, 189, 122, 3, 239, 191, 189, 71, 116, 83, 28, 239, 191, 189, 32, 239, 191, 189, 239, 191, 189, 239, 191, 189, 39, 45, 161, 42, 145, 146, 217, 49, 15, 239, 191, 189, 101, 239, 191, 189, 87, 91, 239, 191, 189, 66, 239, 191, 189, 239, 191, 189, 70, 239, 191, 189, 239, 191, 189, 111, 46, 239, 191, 189, 239, 191, 189, 112, 30, 239, 191, 189, 117, 108, 35, 239, 191, 189, 239, 191, 189, 161, 42, 145, 146, 217, 45, 66, 239, 191, 189, 115, 48, 113, 239, 191, 189, 239, 191, 189, 117, 49, 239, 191, 189, 54, 239, 191, 189, 39, 239, 191, 189, 12, 48, 114, 227, 165, 184, 239, 191, 189, 116, 239, 191, 189, 239, 191, 189, 239, 191, 189, 161, 42, 206, 13, 246, 33, 212, 147, 147, 148, 172, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 0, 172, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 147, 0, 0, 205, 2, 132, 148, 164, 47, 117, 115, 114, 0, 164, 47, 117, 115, 114, 147, 0, 0, 205, 2, 132, 148, 168, 47, 117, 115, 114, 47, 98, 105, 110, 0, 168, 47, 117, 115, 114, 47, 98, 105, 110, 147, 0, 0, 205, 2, 132, 145, 149, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 217, 64, 99, 98, 48, 54, 53, 57, 52, 50, 53, 52, 52, 54, 98, 100, 55, 57, 101, 55, 54, 57, 57, 101, 56, 53, 56, 48, 52, 49, 55, 52, 56, 100, 101, 97, 97, 101, 56, 52, 50, 51, 102, 54, 51, 101, 54, 102, 101, 97, 102, 57, 48, 55, 98, 102, 98, 98, 57, 51, 52, 53, 97, 51, 50, 98, 147, 0, 0, 205, 2, 132, 0, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 145, 147, 183, 47, 104, 101, 108, 108, 111, 95, 119, 111, 114, 108, 100, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100, 0, 179, 47, 117, 115, 114, 47, 98, 105, 110, 47, 104, 101, 108, 108, 111, 119, 111, 114, 108, 100]
    }

    #[allow(unused)]
    pub fn get_test_privkey() -> PrivateKey {
        PrivateKey::from_anonymous(&"AWxDWGKXZZOndWlvY5gvsbLzeRJEFpueNUoR/VCDKXMtBoeIyZoHATvrJWgu5vG2XlEqAbZuUGtCRERaa2aBPw==".to_string()).unwrap()
    }

    #[allow(unused)]
    pub fn get_test_pubkey() -> PublicKey {
        PublicKey::from_anonymous(&"LQaHiMmaBwE76yVoLubxtl5RKgG2blBrQkREWmtmgT8=".to_string()).unwrap()
    }

    #[allow(unused)]
    pub fn get_test_repo_baseurl() -> Url {
        Url::parse("https://example.com/mangrove/tests/").unwrap()
    }

    #[allow(unused)]
    pub fn get_test_repo_repoinfo() -> Url {
        Url::parse("https://example.com/mangrove/tests/repoinfo").unwrap()
    }

    #[allow(unused)]
    pub fn get_test_nonsense_package() -> Package {
        // test_package@v1, if this changes the below byte repr also has to be updated!
        let pkg: Package = Package {
            pkgname: "Y]Ѹ���T^X�*���:39:".to_string(),
            pkgver: Version { major: 999, minor: 999, patch: 999, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY },
            shortdesc: "	Eu�`���	���".to_string(),
            longdesc: Some("$�4COb����� �oXd9@*��Y�}�u�c&n���u>.l,N�՟�-�I����N��<�lt�~���>�N��AR�
�d����b���{0C�y	Iȓ�bj�n��OS�^+���3*�=��Ȏ".to_string()),
            arch: Architecture::amd64,
            url: Some("p9Sd�s�%��H@J�舍�-��$��".to_string()),
            license: Some("�-�c�
                          ���f��RW�*�U1F
                          ".to_string()),
            groups: Some(vec!["4O���K�B�?ЇX�2��.��57�".to_string(), "a͢�L1�3��A��O?}�
k!�h�".to_string()]),
            depends: Some(vec![PkgSpec {
                pkgname: "�?=����k�peq�g8/��x��8s".to_string(),
                version: VersionReq { comparators: vec![] },
            }, PkgSpec {
                pkgname: "�5OB:?
?���|E��|�K��}".to_string(),
                version: VersionReq::parse("153435435.2343452356.3254245435435").unwrap(),
            }]),
            optdepends: Some(vec!["��t~`���^��,��:G��<���e~".to_string()]),
            provides: Some(vec![
                PkgSpec {
                    pkgname: "�Y�O�gǾ��z�GtS� ���'-".to_string(),
                    version: VersionReq { comparators: vec![] },
                }
            ]),
            conflicts: Some(vec![
                PkgSpec {
                    pkgname: "�e�W[�B��F��o.��p�ul#��".to_string(),
                    version: VersionReq { comparators: vec![] },
                }
            ]),
            replaces: Some(vec![
                PkgSpec {
                    pkgname: "B�s0q��u1�6�'�0r㥸�t���".to_string(),
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
}

#[cfg(test)]
mod libmangrove_pkg_tests {
    use std::env;
    use std::fs;
    use std::fs::remove_dir_all;
    use std::path::Path;

    use serial_test::serial;
    use version::{BuildMetadata, Prerelease, Version, VersionReq};

    use crate::crypt::is_signed_package;
    use crate::file::FileOps;
    use crate::pkg::{extract_pkg_to, get_pkg_filename, install_pkg_to, Package, PackageContents, PkgSpec, save_package, save_package_signed};
    use crate::pkgdb::{pkgdb_load, pkgdb_save};
    use crate::platform::Architecture;
    use crate::test::libmangrove_tests_common::{get_test_nonsense_package, get_test_nonsense_package_bytes, get_test_package, get_test_package_bytes, get_test_privkey};
    use crate::version_any;

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
            &get_test_package(),
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
            &get_test_package(),
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
            &get_test_package(),
            format!("{}/../test/test-package", env::current_dir().unwrap().to_str().unwrap()),
            get_test_privkey(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };
        let fname = format!("../test/test-package/{}", get_pkg_filename(&get_test_package()));
        let signed_package_data = fs::read(fname).unwrap();
        assert!(is_signed_package(&signed_package_data));
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

    #[test]
    #[serial]
    fn package_extracting() {
        if Path::new(&format!("{}/../test/fakeroot", env::current_dir().unwrap().to_str().unwrap())).exists() {
            remove_dir_all(format!("{}/../test/fakeroot", env::current_dir().unwrap().to_str().unwrap())).unwrap();
        }
        match save_package(
            &get_test_package(),
            format!("{}/../test/test-package", env::current_dir().unwrap().to_str().unwrap()),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("{}", err);
            }
        }
        let data: Vec<u8> = fs::read(format!("{}/../test/test-package/test_0.0.1_amd64.mgve", env::current_dir().unwrap().to_str().unwrap())).unwrap();
        extract_pkg_to(&data, format!("{}/../test/fakeroot", env::current_dir().unwrap().to_str().unwrap())).unwrap();
    }

    #[test]
    fn package_serialization_nonsense() {
        let pkg = get_test_nonsense_package();
        let serialized: Vec<u8> = rmp_serde::to_vec(&pkg).unwrap();
        println!("{:?}", serialized);
        let expect: Vec<u8> = get_test_nonsense_package_bytes();
        assert_eq!(serialized, expect);
    }

    #[test]
    fn package_deserialization_nonsense() {
        let serialized: Vec<u8> = get_test_nonsense_package_bytes();
        let deserialized: Package = rmp_serde::from_slice(&serialized[..]).unwrap();
        let expect: Package = get_test_nonsense_package();
        assert_eq!(deserialized, expect);
    }

    #[test]
    fn package_save_to_file_does_not_exist_nonsense() {
        // Get a test package
        let package: Package = get_test_nonsense_package();
        // Try to write
        assert!(Package::as_file(&package, "/path/nonexistent-file-nonsense/".to_string()).is_err());
    }

    #[test]
    #[should_panic]
    fn package_save_to_file_without_permissions_nonsense() {
        let package: Package = get_test_nonsense_package();
        Package::as_file(&package, "/root/cant-write-here-nonsense/".to_string()).unwrap();
    }

    #[test]
    #[serial] // If run concurrently with package_saving_signed, will fail
    fn package_saving_nonsense() {
        println!("{:?}", env::current_dir().unwrap());
        match save_package(
            &get_test_nonsense_package(),
            format!("{}/../test/test-package-nonsense", env::current_dir().unwrap().to_str().unwrap()),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    #[serial] // If run concurrently with package_saving, will fail
    fn package_saving_signed_nonsense() {
        match save_package_signed(
            &get_test_nonsense_package(),
            format!("{}/../test/test-package-signed-nonsense", env::current_dir().unwrap().to_str().unwrap()),
            get_test_privkey(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    #[serial]
    fn package_validating_signed_nonsense() {
        match save_package_signed(
            &get_test_nonsense_package(),
            format!("{}/../test/test-package-signed-nonsense", env::current_dir().unwrap().to_str().unwrap()),
            get_test_privkey(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };
        println!("{}", env::current_dir().unwrap().to_str().unwrap());
        let fname = format!("{}/../test/test-package-signed-nonsense/{}", env::current_dir().unwrap().to_str().unwrap(), get_pkg_filename(&get_test_nonsense_package()));
        println!("{}", fname);
        let signed_package_data = fs::read(fname).unwrap();
        assert!(is_signed_package(&signed_package_data));
    }

    #[test]
    fn package_fileops_load_nonsense() {
        match Package::as_file(&get_test_nonsense_package(), "../test/test_pkginfo_nonsense".parse().unwrap()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        let newpkg = match Package::from_file("../test/test_pkginfo_nonsense".parse().unwrap()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(newpkg, get_test_nonsense_package());
    }

    #[test]
    #[serial]
    #[should_panic]
    fn package_installation_dependency_missing() {
        let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        let fakeroot = format!("{}/../test/package-installation-fakeroot", cwd);

        if Path::new(&fakeroot).exists() { remove_dir_all(&fakeroot).unwrap(); }

        save_package(&get_test_package(), format!("{}/../test/package-installation", cwd)).unwrap();

        // lock the database
        let mut db = pkgdb_load(true).unwrap();

        // install the package
        let res = install_pkg_to(&fs::read(format!("{}/../test/package-installation/test_0.0.1_amd64.mgve", cwd)).unwrap(), fakeroot, &mut db);
        // save it
        pkgdb_save(db, true).unwrap();

        res.unwrap()
    }

    #[test]
    #[serial]
    #[should_panic]
    fn package_installation_conflicts_us() {
        let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        let fakeroot = format!("{}/../test/package-installation-fakeroot", cwd);

        if Path::new(&fakeroot).exists() { remove_dir_all(&fakeroot).unwrap(); }

        save_package(&get_test_package(), format!("{}/../test/package-installation", cwd)).unwrap();

        // lock the database
        let mut db = pkgdb_load(true).unwrap();

        let dependency = Package {
            pkgname: "test_2".to_string(),
            pkgver: Version { major: 0, minor: 0, patch: 1, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY },
            shortdesc: "A test package, used in Mangrove unit tests".to_string(),
            longdesc: None,
            arch: Architecture::amd64,
            url: None,
            license: None,
            groups: None,
            depends: None,
            optdepends: None,
            provides: None,
            conflicts: Some(vec![
                PkgSpec {
                    pkgname: "test".to_string(),
                    version: version_any!(),
                }
            ]),
            replaces: None,
            installed_size: 234234324,
            pkgcontents: PackageContents {
                folders: Some(vec![]),
                files: Some(vec![]),
                links: Some(vec![]),
            },
        };
        db.db.installed_packages.push(dependency);

        // install the package
        let res = install_pkg_to(&fs::read(format!("{}/../test/package-installation/test_0.0.1_amd64.mgve", cwd)).unwrap(), fakeroot, &mut db);
        db.db.installed_packages.remove(db.db.installed_packages.len() - 1);
        // save it
        pkgdb_save(db, true).unwrap();

        res.unwrap();
    }

    #[test]
    #[serial]
    #[should_panic]
    fn package_installation_conflicts_them() {
        let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        let fakeroot = format!("{}/../test/package-installation-fakeroot", cwd);

        if Path::new(&fakeroot).exists() { remove_dir_all(&fakeroot).unwrap(); }

        save_package(&get_test_package(), format!("{}/../test/package-installation", cwd)).unwrap();

        // lock the database
        let mut db = pkgdb_load(true).unwrap();

        let dependency = Package {
            pkgname: "conflicting_package".to_string(),
            pkgver: Version { major: 0, minor: 0, patch: 1, pre: Prerelease::EMPTY, build: BuildMetadata::EMPTY },
            shortdesc: "A test package, used in Mangrove unit tests".to_string(),
            longdesc: None,
            arch: Architecture::amd64,
            url: None,
            license: None,
            groups: None,
            depends: None,
            optdepends: None,
            provides: None,
            conflicts: None,
            replaces: None,
            installed_size: 234234324,
            pkgcontents: PackageContents {
                folders: Some(vec![]),
                files: Some(vec![]),
                links: Some(vec![]),
            },
        };
        db.db.installed_packages.push(dependency);

        // install the package
        let res = install_pkg_to(&fs::read(format!("{}/../test/package-installation/test_0.0.1_amd64.mgve", cwd)).unwrap(), fakeroot, &mut db);
        db.db.installed_packages.remove(db.db.installed_packages.len() - 1);
        // save it
        pkgdb_save(db, true).unwrap();

        res.unwrap();
    }
}

#[cfg(test)]
mod libmangrove_repository_tests {
    use crate::repo::get_repoinfo_url;
    use crate::test::libmangrove_tests_common::{get_test_repo_baseurl, get_test_repo_repoinfo};

    #[test]
    pub fn repo_repoinfo_url() {
        println!("{} {}", get_repoinfo_url(get_test_repo_baseurl()).unwrap(), get_test_repo_baseurl());
        assert_eq!(get_repoinfo_url(get_test_repo_baseurl()).unwrap(), get_test_repo_repoinfo());
    }
}

#[cfg(test)]
mod libmangrove_mcrypt_tests {
    use serial_test::serial;

    use crate::aes::{AES128Cipher, AES192Cipher, AES256Cipher};
    use crate::crypt::{debug_dump_package, decrypt_package, encrypt_package, find_key, PrivateKey};
    use crate::test::libmangrove_tests_common::{get_test_package_bytes, get_test_privkey, get_test_pubkey};
    use crate::trustcache::{allow_pk, allow_sk, clear_pk, clear_sk, trustcache_load, trustcache_save};

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
        let decrypted = match decrypt_package(&pk, &encrypted[..]) {
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
        println!("{}", debug_dump_package(&encrypted, Some(&pk)));
        println!("{}", debug_dump_package(&encrypted, None));
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

    #[test]
    #[serial] // Locks the trustcache
    fn mcrypt_find_key() {
        let mut trustcache = trustcache_load(true).unwrap();
        let data = encrypt_package(&get_test_privkey(), &get_test_package_bytes()[..]).unwrap();
        allow_pk(&mut trustcache, &get_test_pubkey()).unwrap();
        let _ = find_key(&data[..], &trustcache).unwrap();
        clear_pk(&mut trustcache, &get_test_pubkey()).unwrap();
        trustcache_save(trustcache, true).unwrap();
    }

    #[test]
    #[serial] // Locks the trustcache
    fn mcrypt_find_key_by_assoc() {
        let mut trustcache = trustcache_load(true).unwrap();
        let data = encrypt_package(&get_test_privkey(), &get_test_package_bytes()[..]).unwrap();
        allow_sk(&mut trustcache, &get_test_privkey()).unwrap();
        let _ = find_key(&data[..], &trustcache).unwrap();
        clear_sk(&mut trustcache, &get_test_privkey()).unwrap();
        trustcache_save(trustcache, true).unwrap();
    }
}

#[cfg(test)]
mod libmangrove_lockfile_tests {
    use serial_test::serial;

    use crate::lock::{lock_packages, lock_repository, lock_trustcache};

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
    use serial_test::serial;

    use crate::config::get_pkgdb_file;
    use crate::pkgdb::{pkgdb_load, pkgdb_save};
    use crate::trustcache::{trustcache_load, trustcache_save};

    #[test]
    #[serial]
    fn trustcache() {
        let trustcache = trustcache_load(true).unwrap();
        trustcache_save(trustcache, true).unwrap();
    }

    #[test]
    #[serial]
    fn test_get_pkgdb_file() {
        assert_eq!(get_pkgdb_file(true), "./db");
        assert_eq!(get_pkgdb_file(false), "/etc/mangrove/db");
    }

    #[test]
    #[serial]
    fn pkgdb() {
        let pkgdb = pkgdb_load(true).unwrap();
        pkgdb_save(pkgdb, true).unwrap();
    }
}