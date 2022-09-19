use std::error::Error;

use clap::{ArgAction, Parser, Subcommand};
use colored::Colorize;

use libmangrove::crypt::{PrivateKey, PublicKey};
use libmangrove::trustcache::{allow_pk, allow_sk, clear_pk, clear_sk, deny_pk, deny_sk, is_pk_blacklisted, is_pk_trusted, is_sk_blacklisted, is_sk_trusted, trustcache_load, trustcache_save};

use crate::{err, ExecutableCommand};
use crate::util::info;

#[derive(Parser)]
#[clap(name = "trust", about = "Manage the Mangrove trustcache", version, author)]
pub struct TrustCommand {
    #[clap(subcommand)]
    pub command: TrustCommandOptions,
}

#[derive(Subcommand)]
pub enum TrustCommandOptions {
    #[clap(name = "allow", help = "Add a key to the trusted section of the trustcache")]
    Allow(TrustCommandAllow),
    #[clap(name = "deny", help = "Add a key to the denied section of the trustcache, removing it from the trusted section if it is present")]
    Deny(TrustCommandDeny),
    #[clap(name = "clear", help = "Remove a key from the trustcache altogether")]
    Clear(TrustCommandClear),
    #[clap(name = "query", help = "Query the current status of a key")]
    Query(TrustCommandQuery)
}

#[derive(Parser)]
#[clap(about = "Allow a public or private key in the trustcache")]
pub struct TrustCommandAllow {
    pub key: String,
    #[clap(short = 'l', long = "local", action = ArgAction::SetTrue, default_value_t = false, help = "Use a local trustcache instead of the default system-wide one")]
    pub local: bool
}
#[derive(Parser)]
#[clap(about = "Blacklist a public or private key in the trustcache")]
pub struct TrustCommandDeny {
    pub key: String,
    #[clap(short = 'l', long = "local", action = ArgAction::SetTrue, default_value_t = false, help = "Use a local trustcache instead of the default system-wide one")]
    pub local: bool
}
#[derive(Parser)]
#[clap(about = "Remove a public or private key from the allow/blocklists if it is present in either")]
pub struct TrustCommandClear {
    pub key: String,
    #[clap(short = 'l', long = "local", action = ArgAction::SetTrue, default_value_t = false, help = "Use a local trustcache instead of the default system-wide one")]
    pub local: bool
}
#[derive(Parser)]
#[clap(about = "Query the current state of a key")]
pub struct TrustCommandQuery {
    pub key: String,
    #[clap(short = 'l', long = "local", action = ArgAction::SetTrue, default_value_t = false, help = "Use a local trustcache instead of the default system-wide one")]
    pub local: bool
}

impl ExecutableCommand for TrustCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        match &self.command {
            TrustCommandOptions::Allow(allow) => allow.execute()?,
            TrustCommandOptions::Deny(deny) => deny.execute()?,
            TrustCommandOptions::Clear(clear) => clear.execute()?,
            TrustCommandOptions::Query(query) => query.execute()?
        }
        Ok(())
    }
}
impl ExecutableCommand for TrustCommandAllow {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info("loading the trustcache".into());
        let mut trustcache = trustcache_load(self.local)?;
        info(format!("adding {} to the trustcache", self.key.blue()));
        // Attempt to determine what the key is
        if let Ok(sk) = PrivateKey::from_anonymous(&self.key) {
            if is_sk_trusted(&trustcache, &sk)? {
                info(format!("{} is already trusted", self.key.blue()));
                trustcache_save(trustcache, self.local)?;
                return Ok(());
            }
            allow_sk(&mut trustcache, &sk)?;
            trustcache_save(trustcache, self.local)?;
            info(format!("added new private key {} to the trustcache", self.key.blue()));
            return Ok(());
        } else if let Ok(pk) = PublicKey::from_anonymous(&self.key) {
            if is_pk_trusted(&trustcache, &pk)? {
                info(format!("{} is already trusted", self.key.blue()));
                trustcache_save(trustcache, self.local)?;
                return Ok(());
            }
            allow_pk(&mut trustcache, &pk)?;
            trustcache_save(trustcache, self.local)?;
            info(format!("added new public key {} to the trustcache", self.key.blue()));
        } else {
            err(format!("could not interpret {} as a public or private key, no changes made", self.key.blue()));
            trustcache_save(trustcache, self.local)?;
        }
        Ok(())
    }
}
impl ExecutableCommand for TrustCommandDeny {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info("loading the trustcache".into());
        let mut trustcache = trustcache_load(self.local)?;
        info(format!("blocking {} in the trustcache", self.key.blue()));
        // Attempt to determine what the key is
        if let Ok(sk) = PrivateKey::from_anonymous(&self.key) {
            if is_sk_blacklisted(&trustcache, &sk)? {
                info(format!("{} is already blacklisted", self.key.blue()));
                trustcache_save(trustcache, self.local)?;
                return Ok(());
            }
            deny_sk(&mut trustcache, &sk)?;
            trustcache_save(trustcache, self.local)?;
            info(format!("added new private key {} to the blacklist", self.key.blue()));
            return Ok(());
        } else if let Ok(pk) = PublicKey::from_anonymous(&self.key) {
            if is_pk_blacklisted(&trustcache, &pk)? {
                info(format!("{} is already blacklisted", self.key.blue()));
                trustcache_save(trustcache, self.local)?;
                return Ok(());
            }
            deny_pk(&mut trustcache, &pk)?;
            trustcache_save(trustcache, self.local)?;
            info(format!("added new public key {} to the blacklist", self.key.blue()));
        } else {
            err(format!("could not interpret {} as a public or private key, no changes made", self.key.blue()));
            trustcache_save(trustcache, self.local)?;
        }
        Ok(())
    }
}
impl ExecutableCommand for TrustCommandClear {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info("loading the trustcache".into());
        let mut trustcache = trustcache_load(self.local)?;
        info(format!("clearing {} from the trustcache", self.key.blue()));
        // Attempt to determine what the key is
        if let Ok(sk) = PrivateKey::from_anonymous(&self.key) {
            clear_sk(&mut trustcache, &sk)?;
            trustcache_save(trustcache, self.local)?;
            info(format!("removed key {} from the trustcache", self.key.blue()));
            return Ok(());
        } else if let Ok(pk) = PublicKey::from_anonymous(&self.key) {
            clear_pk(&mut trustcache, &pk)?;
            trustcache_save(trustcache, self.local)?;
            info(format!("removed key {} from the trustcache", self.key.blue()));
        } else {
            err(format!("could not interpret {} as a public or private key, no changes made", self.key.blue()));
            trustcache_save(trustcache, self.local)?;
        }
        Ok(())
    }
}
impl ExecutableCommand for TrustCommandQuery {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info("loading the trustcache".into());
        let trustcache = trustcache_load(self.local)?;
        return if let Ok(sk) = PrivateKey::from_anonymous(&self.key) {
            if is_sk_trusted(&trustcache, &sk)? {
                info(format!("{} is trusted", self.key.blue()));
            } else if is_sk_blacklisted(&trustcache, &sk)? {
                info(format!("{} is blacklisted", self.key.blue()));
            } else {
                info(format!("{} is not present in the trustcache", self.key.blue()));
            }
            trustcache_save(trustcache, self.local)?;
            Ok(())
        } else if let Ok(pk) = PublicKey::from_anonymous(&self.key) {
            if is_pk_trusted(&trustcache, &pk)? {
                info(format!("{} is trusted", self.key.blue()));
            } else if is_pk_blacklisted(&trustcache, &pk)? {
                info(format!("{} is blacklisted", self.key.blue()));
            } else {
                info(format!("{} is not present in the trustcache", self.key.blue()));
            }
            trustcache_save(trustcache, self.local)?;
            Ok(())
        } else {
            err(format!("could not interpret {} as a public or private key", self.key.blue()));
            trustcache_save(trustcache, self.local)?;
            Ok(())
        }
    }
}