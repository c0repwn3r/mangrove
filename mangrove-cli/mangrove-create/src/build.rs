use std::{fs, io};
use std::error::Error;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use libmangrove::crypt::mcrypt_sha256_file;
use libmangrove::pkg::{FileMetadata, Package, PackageContents, PackageFile, PackageFolder, PackageLink, save_package};
use crate::BuildConfig;

pub struct BuiltPackageContents {
    pub pkgcontents: PackageContents,
    pub size: usize
}

pub fn get_pkgcontents(dir: &Path, cut: bool, root: &Path) -> Result<BuiltPackageContents, Box<dyn Error>> {
    let mut result: BuiltPackageContents = BuiltPackageContents {
        pkgcontents: PackageContents {
            folders: Some(vec![]),
            files: Some(vec![]),
            links: Some(vec![])
        },
        size: 0
    };
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let mut pkgpath = path.to_str().unwrap().to_owned();
            if pkgpath.starts_with(root.to_str().unwrap()) {
                pkgpath = pkgpath[root.to_str().unwrap().len()..].parse().unwrap();
            }
            if cut && pkgpath.starts_with(dir.to_str().unwrap()) {
                pkgpath = pkgpath[dir.to_str().unwrap().len()..].parse().unwrap();
            }

            if path.is_dir() {
                println!("folder: {} -> {}", path.to_str().unwrap(), pkgpath);
                result.pkgcontents.folders.as_mut().unwrap().push(PackageFolder {
                    name: pkgpath.clone(),
                    mtime: path.metadata().unwrap().modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
                    installpath: pkgpath,
                    meta: FileMetadata {
                        owner: path.metadata().unwrap().uid() as usize,
                        group: path.metadata().unwrap().gid() as usize,
                        permissions: path.metadata().unwrap().permissions().mode() as usize
                    }
                });
                let mut res = get_pkgcontents(&path, false, root).expect("recursive call failed");
                result.size += res.size;
                if res.pkgcontents.links.is_some() {
                    result.pkgcontents.links.as_mut().unwrap().append(&mut res.pkgcontents.links.unwrap());
                }
                if res.pkgcontents.files.is_some() {
                    result.pkgcontents.files.as_mut().unwrap().append(&mut res.pkgcontents.files.unwrap());
                }
                if res.pkgcontents.folders.is_some() {
                    result.pkgcontents.folders.as_mut().unwrap().append(&mut res.pkgcontents.folders.unwrap());
                }
            } else if path.is_file() {
                if path.to_str().unwrap().contains(".mgve.toml") {
                    continue;
                }
                if path.to_str().unwrap().ends_with(".mgve") {
                    continue;
                }
                println!("file: {} -> {}", path.to_str().unwrap(), pkgpath);
                result.pkgcontents.files.as_mut().unwrap().push(PackageFile {
                    name: pkgpath.clone(),
                    sha256: mcrypt_sha256_file(&path.to_str().unwrap().to_string())?,
                    meta: FileMetadata {
                        owner: path.metadata().unwrap().uid() as usize,
                        group: path.metadata().unwrap().gid() as usize,
                        permissions: path.metadata().unwrap().permissions().mode() as usize
                    },
                    mtime: path.metadata().unwrap().modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
                    installpath: pkgpath
                });
                result.size += path.metadata().unwrap().size() as usize;
            } else if path.is_symlink() {
                let mut pkgpath_target = path.read_link().unwrap().to_str().unwrap().to_string();
                if pkgpath_target.starts_with(root.to_str().unwrap()) {
                    pkgpath_target = pkgpath_target[root.to_str().unwrap().len()..].parse().unwrap();
                }
                if cut && pkgpath_target.starts_with(dir.to_str().unwrap()) {
                    pkgpath_target = pkgpath_target[dir.to_str().unwrap().len()..].parse().unwrap();
                }
                println!("link: {}@{} -> {}@{}", path.to_str().unwrap(), path.read_link().unwrap().to_str().unwrap(), pkgpath, pkgpath_target);
                result.pkgcontents.links.as_mut().unwrap().push(PackageLink {
                    file: pkgpath,
                    mtime: path.symlink_metadata().unwrap().modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
                    target: pkgpath_target
                });
            }
        }
    }
    Ok(result)
}

pub fn build(config: BuildConfig, dryrun: bool) {
    // convert to Package
    let contents = match get_pkgcontents(Path::new("."), true, Path::new(".")) {
        Ok(c) => c,
        Err(e) => {
            println!("failed to enumerate package contents: {}", e);
            std::process::exit(22);
        }
    };

    let pkgcontents = contents.pkgcontents;
    let size = contents.size;

    println!("{:?}", pkgcontents);
    println!("{}", size);

    // get all files


    let pkg: Package = Package {
        pkgname: config.pkgname,
        pkgver: config.pkgver,
        shortdesc: config.shortdesc,
        longdesc: config.longdesc,
        arch: config.arch,
        url: config.url,
        license: config.license,
        groups: config.groups,
        depends: config.depends,
        optdepends: config.optdepends,
        provides: config.provides,
        conflicts: config.conflicts,
        replaces: config.replaces,
        installed_size: size,
        pkgcontents
    };

    println!("{:?}", pkg);
    save_package(pkg, fs::canonicalize(PathBuf::from(".")).unwrap().to_str().unwrap().to_string()).unwrap();
}