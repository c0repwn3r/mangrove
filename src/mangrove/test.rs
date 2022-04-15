#[cfg(test)]
mod tests {
    use crate::mangrove::{pkg::{Package, PackageContents, PkgSpec, PackageFolder, PackageFile, FileMetadata, PackageLink}, version::{Version, VersionRange}, platform::Architecture};

    fn get_test_package() -> Package {
        let pkg: Package = Package {
            pkgname: "test".to_string(),
            pkgver: Version { major: 0, minor: 0, patch: 1, prerelease: None, build: None },
            shortdesc: "A test package, used in Mangrove unit tests".to_string(),
            longdesc: Some("This is a longer package description for test, which is a test package uesd in mangrove unit tests.".to_string()),
            arch: Architecture::Amd64,
            url: Some("https://mgve.cc".to_string()),
            license: Some("GNU-GPL-3-or-later".to_string()),
            groups: Some(vec!["thisisgroup1".to_string(), "thisisgroup2".to_string()]),
            depends: Some(vec![PkgSpec {
                pkgname: "test-data".to_string(),
                version: VersionRange {
                    any: true,
                    minver: None,
                    maxver: None
                }
            }, PkgSpec {
                pkgname: "test-data-2".to_string(),
                version: VersionRange {
                    any: false,
                    minver: Some(Version { major: 0, minor: 0, patch: 0, prerelease: None, build: None }),
                    maxver: Some(Version { major: 0, minor: 0, patch: 0, prerelease: None, build: None })
                }
            }]),
            optdepends: Some(vec!["test-opt: for doing something else".to_string()]),
            provides: Some(vec![
                PkgSpec {
                    pkgname: "other-package".to_string(),
                    version: VersionRange {
                        any: true,
                        minver: None,
                        maxver: None
                    }
                }
            ]),
            conflicts: Some(vec![
                PkgSpec {
                    pkgname: "conflicting-package".to_string(),
                    version: VersionRange {
                        any: true,
                        minver: None,
                        maxver: None
                    }
                }
            ]),
            replaces: Some(vec![
                PkgSpec {
                    pkgname: "old-package".to_string(),
                    version: VersionRange {
                        any: true,
                        minver: None,
                        maxver: None
                    }
                }
            ]),
            installed_size: 234234324,
            pkgcontents: PackageContents {
                folders: Some(vec![
                    PackageFolder {
                        name: "/hello_world".to_string(),
                        mtime: 0,
                        installpath: "/hello_world".to_string()
                    },
                    PackageFolder {
                        name: "/usr".to_string(),
                        mtime: 0,
                        installpath: "/usr".to_string()
                    },
                    PackageFolder {
                        name: "/usr/bin".to_string(),
                        mtime: 0,
                        installpath: "/usr/bin".to_string()
                    }
                ]),
                files: Some(vec![
                    PackageFile {
                        name: "/hello_world/helloworld".to_string(),
                        sha256: "f64f5bd7f162c81813b4a2bf7993e9bfe9f821087b7ce62f76fe399f727292b0".to_string(),
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

        return pkg;
    }

    #[test]
    fn package_serialization() {
        let pkg = get_test_package();
        let serialized: Vec<u8> = rmp_serde::to_vec(&pkg).unwrap();
        let serialized_json: String = serde_json::to_string(&pkg).unwrap();
        println!("sizeof json: {}, real {}", serialized_json.len(), serialized.len());
        println!("{}", serialized_json);
        println!("{:x?}", serialized);
        let deserialized: Package = rmp_serde::from_slice(&serialized[..]).unwrap();
        assert_eq!(pkg, deserialized);
    }
}