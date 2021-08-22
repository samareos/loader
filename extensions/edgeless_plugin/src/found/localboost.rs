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

use std::path::*;
use std::ops::{Deref, DerefMut};
use anyhow::anyhow;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct BoostPluginEntry {
  pub path: PathBuf,
  pub meta: Option<PluginMetadata>,
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

impl Into<ProfileEntry> for BoostRepoEntry {
  fn into(self) -> ProfileEntry {
      self.profile
  }
}

impl BoostRepoEntry {
  pub async fn new(profile: ProfileEntry) -> anyhow::Result<Self> {
    if profile.profile_type == ProfileType::Default {
      return Err(anyhow!("this profile is not a localboost repo"));
    }

    let mut entries = vec![];
    let lb_path = profile.path.join(&PATH_PLUGIN_LB_RESOURCES.as_path());

    let mut f_iter = fs::read_dir(lb_path).await?;
    
    loop {
      if let Some(f_entry) = f_iter.next_entry().await? {
        let meta = f_entry.metadata().await?;
        if meta.is_dir() {
          entries.push(BoostPluginEntry {
            path: f_entry.path(),
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
      Self {
        profile,
        plugins: entries,
      }
    )
  }
}

#[cfg(test)]
mod tests {
    use edgeless_core::found::ProfileEntry;

    use super::BoostRepoEntry;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    let entries = ProfileEntry::find_boostrepo().await?;
    if let Some(profile) = entries.last() {
      let boost = BoostRepoEntry::new(profile.to_owned()).await?;
      println!("{:#?}", boost);
    }

    Ok(())
  }
}