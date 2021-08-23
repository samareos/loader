#![allow(unused_imports)]
use edgeless_core::found::{
  ProfileEntry,
  ProfileType
};
use edgeless_core::options::define::{
  PATH_PLUGIN_RESOURCES,
  PATH_PLUGIN_LB_RESOURCES,
  EXT_PLUGIN_LOCALBOOST,
};
use super::{
  PluginEntry,
  PluginExtension,
  PluginMetadata,
};
use std::collections::HashMap;
use std::rc::Rc;

use std::path::*;
use std::ops::{Deref, DerefMut};
use anyhow::anyhow;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct BoostPluginEntry {
  pub path: PathBuf,
  pub meta: Option<PluginMetadata>,
  pub from: PathBuf,
}

#[derive(Debug, Clone)]
pub struct BoostRepoEntry {
  pub profile: ProfileEntry,
  pub plugins: Vec<BoostPluginEntry>
}

impl Deref for BoostRepoEntry {
  type Target = ProfileEntry;

  fn deref(&self) -> &Self::Target {
    &self.profile
  }
}

impl DerefMut for BoostRepoEntry {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.profile
  }
}

impl Into<Vec<BoostPluginEntry>> for BoostRepoEntry {
  fn into(self) -> Vec<BoostPluginEntry> {
      self.plugins
  }
}

impl Into<ProfileEntry> for BoostRepoEntry {
  fn into(self) -> ProfileEntry {
      self.profile
  }
}

impl BoostRepoEntry {
  pub async fn find() -> anyhow::Result<BoostRepoEntries> {
    let p = ProfileEntry::find_boostrepo().await?;
    let mut r = vec![];
    for i in p {
      r.push(Self::new(i).await?);
    }

    Ok(BoostRepoEntries(r))
  }
  pub async fn new(profile: ProfileEntry) -> anyhow::Result<BoostRepoEntry> {
    if profile.profile_type == ProfileType::Default {
      return Err(anyhow!("this profile is not a localboost repo"));
    }

    
    let lb_path = profile.path.join(&PATH_PLUGIN_LB_RESOURCES.as_path());

    let mut entry = Self {
      profile,
      plugins: vec![],
    };

    let mut f_iter = fs::read_dir(lb_path).await?;
    
    loop {
      if let Some(f_entry) = f_iter.next_entry().await? {
        let meta = f_entry.metadata().await?;
        if meta.is_dir() {
          entry.plugins.push(BoostPluginEntry {
            path: f_entry.path(),
            from: entry.path.to_path_buf(),
            meta: f_entry.path()
              .file_name()
              .map(|s| 
                PluginMetadata::parse(s.to_str().unwrap_or(""))
              ).flatten()
          });
        }
      } else {
        break;
      }
    }

    Ok(
      entry
    )
  }
}

#[derive(Debug, Clone)]
pub struct BoostRepoEntries (pub Vec<BoostRepoEntry>);
impl From<Vec<BoostRepoEntry>> for BoostRepoEntries {
  fn from(v: Vec<BoostRepoEntry>) -> Self {
      Self(v)
  }
}

impl Deref for BoostRepoEntries {
  type Target = Vec<BoostRepoEntry>;
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl DerefMut for BoostRepoEntries {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

impl Into<Vec<BoostRepoEntry>> for BoostRepoEntries {
  fn into(self) -> Vec<BoostRepoEntry> {
    self.0
  }
}

impl BoostRepoEntries {
  pub fn get_all_plugins(&self) -> Vec<&BoostPluginEntry> {
    let mut p = vec![];
    for i in &self.0 {
      for i in &i.plugins {
        p.push(i);
      }
    }
    p
  }
}


pub type BoostRepoPluginMap<'p> = HashMap<&'p Option<PluginMetadata>, Vec<&'p BoostRepoEntry>>;
impl BoostRepoEntries {
  pub fn get_plugins(&self) -> BoostRepoPluginMap {
    let mut m = HashMap::new();
    for i in &self.0 {
      for p in &i.plugins {
        m.entry(&p.meta).or_insert(vec![]).push(i);
      }
    }
    m
  }
}

#[cfg(test)]
mod tests {
    use edgeless_core::found::ProfileEntry;

    use super::BoostRepoEntry;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {

    let boost = BoostRepoEntry::find().await?;
    let p = boost.get_plugins();
    println!("{:#?}", p);
    println!("{:#?}", boost);

    Ok(())
  }
}