use std::error::Error;
use std::path::PathBuf;

use clap::{ArgAction, Parser};

use libmangrove::crypt::PrivateKey;
use libmangrove::trustcache::{trustcache_load, trustcache_save};

use crate::{err, ExecutableCommand};
use crate::util::{info, sign_pkg};

#[derive(Parser)]
#[clap(name = "sign", about = "Taking an unsigned package, sign it using the provided private key", version, author)]
pub struct SignCommand {
    #[clap(name = "file", value_parser, help = "Unsigned package file to sign")]
    pub file: PathBuf,

    #[clap(name = "key", short = 'k', long = "key", help = "Which private key to use. This may also be a prefix of a key, to use a key from a trustcache. If not provided, will use the first key found in the trustcache.", value_parser)]
    pub key: Option<String>,

    #[clap(name = "output", short = 'o', long = "output", help = "The file to output the signed package to. Defaults to the same file as the unsigned package.", value_parser)]
    pub output_file: Option<PathBuf>,

    #[clap(name = "local", short = 'l', long = "local", help = "Use a local trustcache", action = ArgAction::SetTrue, default_value_t = false, value_parser)]
    pub local: bool
}

impl ExecutableCommand for SignCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let infile = &self.file;

        let outfile = match &self.output_file {
            Some(o) => o,
            None => infile
        };

        // keyfinding logic
        let mut key: Option<PrivateKey> = None;
        info("loading trustcache".into());
        let trustcache = trustcache_load(self.local)?;
        if let Some(ki) = &self.key {
            if let Ok(key_i) = PrivateKey::from_anonymous(ki) {
                if trustcache.keydb.deny_privkeys.contains(ki) {
                    err("this private key has been explicitly blacklisted".into());
                    trustcache_save(trustcache, self.local)?;
                    return Ok(());
                } else {
                    info("loaded private key from cli".into());
                    trustcache_save(trustcache, self.local)?;
                    key = Some(key_i);
                }
            } else {
                if let Some(key_ik) = trustcache.keydb.known_privkeys.iter().find(|k| k.starts_with(ki)) {
                    let key_ikc = key_ik.to_owned();
                    trustcache_save(trustcache, self.local)?;
                    key = Some(PrivateKey::from_anonymous(&key_ikc)?);
                }
            }
        } else {
            if let Some(key_i) = trustcache.keydb.known_privkeys.get(0) {
                let key_ic = key_i.to_owned();
                trustcache_save(trustcache, self.local)?;
                key = Some(PrivateKey::from_anonymous(&key_ic)?);
            } else {
                err("no keys available to sign".into());
            }
        }

        info("creating encrypted package file".into());

        if let Some(kd) = key {
            sign_pkg(infile, outfile, &kd)?;
        } else {
            err("no keys available to sign".into());
        }


        Ok(())
    }
}