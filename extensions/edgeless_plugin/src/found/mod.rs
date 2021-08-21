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
    info!("try parse {:?} to plugin meta", s);
    let ul_sep = s.split("_").collect::<Vec<_>>();
    info!("sep = {:?}", ul_sep);
    info!("sep len = {}", ul_sep.len());
    if ul_sep.len() < 3 || ul_sep.len() > 4 {
      error!("parse to meta error!");
      return None;
    }
    
    let meta = PluginMetadata {
      name: ul_sep[0].to_string(),
      version: ul_sep[1].to_string(),
      author: ul_sep[2].to_string(),
      category: ul_sep.get(3).map(|s| s.to_string())
    };

    info!("parsed, meta = {:#?}", meta);

    Some(meta)
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
  pub fn new(ext: &str) -> Option<Self> {
    info!("parse plugin ext, ext = {:?}", ext);
    if !EXT_PLUGIN_PATTERN.is_match(ext) {
      warn!("invaild plugin ext");
      return None;
    }
    let o = match ext {
      "7z" => Self::Normal,
      "7zl" => Self::Localboost,
      "7zf" => Self::Disabled,
      _ => Self::Unknown,
    };

    info!("parsed, {:?}", o);

    Some(o)
  }
}

#[derive(Debug, Clone)]
pub struct PluginEntry {
  pub path: PathBuf,
  pub extension: Option<PluginExtension>,
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

  pub async fn new_from_profile(entry: ProfileEntry) -> anyhow::Result<Vec<Self>> {
    let res_pb = entry.path.join(PATH_PLUGIN_RESOURCES.clone());

    if !(res_pb.exists() && res_pb.is_dir()) {
      return Err(anyhow!("not found plugin resource in {:#?}", entry));
    }

    let mut dir= fs::read_dir(res_pb).await?;
    let mut plugins = vec![];

    loop {
      if let Some(d) = dir.next_entry().await? {
        let meta = d.metadata().await?;
        if meta.is_file() {
          let r = Self::new(
              d.path()
          ).await?;
          if let Some(_) = r.extension {
            plugins.push(r);
          }
        }
      } else {
        break;
      }
    }

    Ok(plugins)
  }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use edgeless_core::found::ProfileEntry;

    use super::PluginEntry;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    let profile = ProfileEntry::find_last().await?;
    if let Some(profile) = profile {
      let entries = PluginEntry::new_from_profile(profile).await?;
      println!("{:#?}", entries);
    }
    Ok(())
  }
}