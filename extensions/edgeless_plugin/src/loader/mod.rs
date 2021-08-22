#![allow(unused_imports)]

use anyhow::anyhow;
use tokio::fs;
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

  pub dest: PathBuf,
  pub release: PathBuf,
  pub scripts: Vec<PluginEntry>,
  pub depend_files: Vec<PathBuf>,
  pub depend_dirs: Vec<PathBuf>,
}

impl<'a> PluginLoadSession<'a> {
  pub fn new(entry: &'a PluginEntry) -> PluginLoadSession<'a> {
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
    }
  }

  pub fn config(mut self, config: Option<PluginLoadConfig>) -> Self {
    self.config = config;
    self
  }
}


#[cfg(test)]
mod tests {

  use edgeless_core::found::ProfileEntry;
  use crate::found::PluginEntry;

  use super::PluginLoadSession;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    if let Some(profile) = ProfileEntry::find_last().await? {
      let plugins = PluginEntry::from_profile(&profile).await?;
      let session = plugins.iter().map(PluginLoadSession::new).collect::<Vec<_>>();
      println!("{:#?}", session);
    }

    Ok(())
  }
}