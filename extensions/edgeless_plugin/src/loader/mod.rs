#![allow(unused_imports)]

use anyhow::anyhow;
use tokio::fs::{self, DirEntry};
use crate::found::localboost::{BoostPluginEntry, BoostRepoEntries, BoostRepoEntry, BoostRepoPluginMap};
use crate::found::{
  PluginExtension,
  PluginEntry,
  PluginMetadata
};
use bindings_7z::SevenZip;
use bindings_pecmd::Pecmd;
use log::{error, warn, log};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::os::windows::prelude::MetadataExt;
use std::str::FromStr;
use std::vec;
use std::{env::var, path::PathBuf};
use uuid::Uuid;

use edgeless_utils::rand_uuid;

#[derive(Debug, Clone, Copy)]
pub enum PluginScriptType {
  Batch,
  Pecmd,
}

#[derive(Debug, Clone)]
pub struct PluginScriptEntry {
  pub script_type: PluginScriptType,
  pub path_original: PathBuf,
  pub path_mangled: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum PluginLoadState {
  Pending,
  Released,
  Resolved,
  Rejected,
}

#[derive(Debug, Clone, Copy)]
pub enum PluginLoadTarget {
  Normal,
  Localboost,
  Disabled,
}

impl From<PluginExtension> for PluginLoadTarget {
  fn from(ext: PluginExtension) -> Self {
    match ext {
      PluginExtension::Normal => PluginLoadTarget::Normal,
      PluginExtension::Localboost => PluginLoadTarget::Localboost,
      _ => PluginLoadTarget::Disabled,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PluginLoadConfig {
  release: PathBuf,
  dest: PathBuf,
  mangle_id: Uuid,
}

impl PluginLoadConfig {
  pub async fn default() -> anyhow::Result<Self> {
    let dest = PathBuf::from_str(
      &var("PROGRAMFILES")?
    )?.join("Edgeless");

    let release = dest.join("__release__");
    fs::create_dir_all(
      &release
    ).await?;

    Ok(Self {
      release,
      dest,
      mangle_id: rand_uuid().await?,
    })
  }
}


#[derive(Debug, Clone)]
pub struct PluginLoadSession<'a> {
  pub entry: &'a PluginEntry,

  pub target: PluginLoadTarget,
  pub state: PluginLoadState,

  pub config: Option<PluginLoadConfig>,

  pub boostrepo: &'a BoostRepoPluginMap<'a>,

  pub dest: PathBuf,
  pub release: PathBuf,
  pub scripts: Vec<PluginScriptEntry>,
  pub depend_files: Vec<PathBuf>,
  pub depend_dirs: Vec<PathBuf>,
}

impl<'a> PluginLoadSession<'a> {
  pub fn new(entry: &'a PluginEntry, repo: &'a BoostRepoPluginMap) -> PluginLoadSession<'a> {
    let target = entry.extension.clone()
      .unwrap_or(PluginExtension::Disabled)
      .into();
    Self {
      target,
      entry,
      config: None,
      state: PluginLoadState::Pending,
      dest: PathBuf::new(),
      release: PathBuf::new(),
      scripts: vec![],
      depend_files: vec![],
      depend_dirs: vec![],
      boostrepo: repo,
    }
  }

}

impl<'a> PluginLoadSession<'a> {
  pub fn with_config(&mut self, config: Option<PluginLoadConfig>) -> &mut Self {
    self.config = config;
    self
  }

  pub async fn release_as_localboost(&self, entry: &mut BoostRepoEntry, force: bool) -> anyhow::Result<bool> {
    if let Some(m) = self.boostrepo.get(&self.entry.meta) {
      if m.contains(entry) || !force {
        return Ok(false);
      }
    }

    let zip = SevenZip::new()?;
    zip.
    
    todo!()
  }

  pub async fn link_as_localboost(&mut self) -> anyhow::Result<()> {
    todo!()
  }

  pub async fn load_as_normal(&mut self) -> anyhow::Result<()> {
    todo!()
  }

  pub async fn release_as_normal(&mut self) -> anyhow::Result<()> {
    todo!()
  }

  pub async fn load(&mut self) -> anyhow::Result<()> {
    todo!()
  }
}


#[cfg(test)]
mod tests {

  use edgeless_core::found::ProfileEntry;
  use crate::found::{PluginEntry, localboost::BoostRepoEntry};

  use super::PluginLoadSession;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    if let Some(profile) = ProfileEntry::find_last().await? {
      let plugins = PluginEntry::from_profile(&profile).await?;
      let lb = BoostRepoEntry::find().await?;
      let lb = lb.get_plugins();
      let session = plugins.iter()
        .map(
          |v| PluginLoadSession::new(
            v, 
            &lb
          )
        ).collect::<Vec<_>>();
      println!("{:#?}", session);
    }

    Ok(())
  }
}