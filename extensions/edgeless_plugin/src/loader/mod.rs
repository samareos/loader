#![allow(unused_imports)]

use tokio::fs;
use uuid::Builder;
use crate::found::{
  PluginExtension,
  PluginEntry,
  PluginMetadata
};
use rand::thread_rng;
use bindings_7z::SevenZip;
use bindings_pecmd::Pecmd;
use log::{error, warn, log};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::vec;
use std::{env::var, path::PathBuf};

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
pub struct PluginLoadSession {
  pub entry: PluginEntry,

  pub target: PluginLoadTarget,
  pub state: PluginLoadState,

  pub dest: PathBuf,
  pub release: PathBuf,
  pub scripts: Vec<PluginEntry>,
  pub depend_files: Vec<PathBuf>,
  pub depend_dirs: Vec<PathBuf>,
}

impl PluginLoadSession {
  pub fn new(entry: PluginEntry) -> Self {
    let target = entry.extension.clone()
      .unwrap_or(PluginExtension::Unknown)
      .into();
    Self {
      entry,
      target,
      state: PluginLoadState::Pending,
      dest: PathBuf::new(),
      release: PathBuf::new(),
      scripts: vec![],
      depend_files: vec![],
      depend_dirs: vec![],
    }
  }
}