use std::{ffi::{OsString}, path::PathBuf, vec};
use sysinfo::{DiskExt, DiskType, RefreshKind, System, SystemExt};
use tokio::fs;
use edgeless_utils::u8_to_ascii;

use lazy_static::lazy_static;
use log::{info};

lazy_static! {
  static ref UNKNOWN_FS: String = "UNKNOWN".to_string();
  pub static ref PROFILE_PATH: PathBuf = PathBuf::from("Edgeless");
  pub static ref PROFILE_EXIST_PATH: PathBuf = PathBuf::from("Edgeless").join("version.txt");
  pub static ref PROFILE_VER_PATH: PathBuf = PathBuf::from("Edgeless").join("version.txt");
}
#[derive(Debug, Clone)]
pub struct ProfileEntry {
  pub version_text: String,
  pub path: PathBuf,
  pub mountpoint: PathBuf,
  pub name: OsString,
  pub disk_type: DiskType,
  pub removable: bool,
  pub fs: String,
}

impl ProfileEntry {
  pub async fn find_last() -> anyhow::Result<Option<Self>> {
    let all = Self::find().await?;
    info!("get last one to return");
    if let Some(last) = all.last() {
      Ok(Some(last.to_owned()))
    } else {
      info!("not found edgeless profiles");
      Ok(None)
    }
  }

  pub async fn find() -> anyhow::Result<Vec<Self>> {
    let sys = System::new_with_specifics(
      RefreshKind::new().with_disks().with_disks_list()
    );
    info!("refresh disk info");
  
    let mut profiles = vec![];
  
    for i in sys.disks() {
      info!("scanning disk {:?}", i); 
      if i.mount_point().join(&PROFILE_EXIST_PATH.as_path()).exists() {
        info!("found disk `{:?}` has edgeless profile", i.mount_point());
        let version_text = 
          fs::read_to_string(i.mount_point().join(&PROFILE_VER_PATH.as_path()))
            .await.unwrap_or(String::new());
  
        let profile = Self {
          path: i.mount_point().join(&PROFILE_PATH.as_path()),
          version_text,
          mountpoint: i.mount_point().to_path_buf(),
          name: i.name().to_os_string(),
          disk_type: i.type_(),
          removable: i.is_removable(),
          
          fs: u8_to_ascii(i.file_system())
            .unwrap_or(UNKNOWN_FS.clone()),
        };
  
        info!("profile info: {:#?}", profile);

        profiles.push(profile);
      }
    }
    info!("get profiles: {:#?}", profiles);
    Ok(profiles)
  }
}

#[cfg(test)]
mod tests {
    use crate::found::ProfileEntry;

    use log::LevelFilter;
    use log::debug;

    fn init() {
      let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .is_test(true)
        .try_init();
    }
  

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        init();
        debug!("{:#?}", ProfileEntry::find().await?);
        Ok(())
    }
}
