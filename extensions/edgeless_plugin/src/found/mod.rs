#![allow(unused_imports)]
use std::{ffi::OsString, fs::Metadata, path::{Path, PathBuf}};

use edgeless_core::found::ProfileEntry;
pub use edgeless_core::options::define::{
  PATH_PLUGIN_RESOURCES,
  EXT_PLUGIN_DISABLE,
  EXT_PLUGIN_LOCALBOOST,
  EXT_PLUGIN_NORMAL,
  EXT_PLUGIN_PATTERN,
  PATH_EXTERNAL_LAUNCHER
};

use anyhow::anyhow;
use log::{info, warn, error, debug};
use tokio::fs;

#[derive(Debug, Clone)]
pub struct PluginMetadata {
  pub name: String,
  pub version: String,
  pub author: String,
  pub category: Option<String>,
}

impl PluginMetadata {
  pub fn parse(s: &str) -> Option<Self> {
    let ul_sep = s.split("_").collect::<Vec<_>>();
    if ul_sep.len() < 3 || ul_sep.len() > 4 {
      return None;
    }
    
    Some(PluginMetadata {
      name: ul_sep[0].to_string(),
      version: ul_sep[1].to_string(),
      author: ul_sep[2].to_string(),
      category: ul_sep.get(3).map(|s| s.to_string())
    })
  }
}

#[derive(Debug, Clone, Copy)]
pub enum PluginExtension {
  Normal,
  Localboost,
  Disabled,
  Unknown,
  Invaild,
}

impl PluginExtension {
  pub fn new(ext: &str) -> Self {
    if !EXT_PLUGIN_PATTERN.is_match(ext) {
      return Self::Invaild;
    }
    match ext {
      "7z" => Self::Normal,
      "7zl" => Self::Localboost,
      "7zf" => Self::Disabled,
      _ => Self::Unknown,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PluginEntry {
  pub path: PathBuf,
  pub extension: PluginExtension,
  pub meta: Option<PluginMetadata>,
  pub filemeta: Metadata,
}

impl PluginEntry {
  pub async fn new(pb: PathBuf) -> anyhow::Result<Self> {
    if !(pb.exists() && pb.is_file()) {
      return Err(anyhow!("not found this file"));
    }

    let ext = pb.extension().ok_or(
      anyhow!("invalid extension")
    )?.to_string_lossy();

    let ext = PluginExtension::new(&ext);

    let f = fs::File::open(&pb).await?;
    let f = f.metadata().await?;

    let s = pb.file_stem().map(
      |v| v.to_str().map(PluginMetadata::parse).flatten()
    ).flatten();

    Ok(Self {
      path: pb,
      extension: ext,
      meta: s,
      filemeta: f,
    })
  }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::PluginEntry;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    let entry = PluginEntry::new(
      PathBuf::from_str(r"D:\1\Diskgenius专业版_5.2.1.941_Horatio Chow.7z")?
    ).await?;

    println!("{:#?}", entry);

    Ok(())
  }
}