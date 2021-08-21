pub mod define;

use crate::options::define::PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS;
use crate::options::define::PATH_OPTIONS;
use log::{info, warn};
use define::ProfileOption;
use crate::found::ProfileEntry;

use tokio::fs;

#[derive(Debug, Clone)]
pub enum ProfileOptionValue {
  /*
   * w1024 h768 b32 f60
   * 格式：宽(w) 高(h) 色位(b) 刷新率(f)
   */
  CustomDisplayRes(usize, usize, usize, usize),

  /*
   * (url: String, disabled: bool)
   */
  CustomHomepageUrl(String),
  Enabled,
  Disabled,
}

impl From<bool> for ProfileOptionValue {
  fn from(v: bool) -> Self {
    match v {
      true => Self::Enabled,
      false => Self::Disabled,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ProfileOptions {
  pub allow_external_laucher: ProfileOptionValue,
  pub ignore_outdate: ProfileOptionValue,
  pub disable_usb_manager: ProfileOptionValue,
  pub disable_smart_iso: ProfileOptionValue,
  pub unfold_ribbon: ProfileOptionValue,
  pub reboot_default: ProfileOptionValue,
  pub disable_recycle_bin: ProfileOptionValue,
  pub auto_unattend: ProfileOptionValue,
  pub drive_up_active: ProfileOptionValue,
  pub drive_windows_first: ProfileOptionValue,
  pub drive_order_another_way: ProfileOptionValue,
  pub drive_mount_every_partition: ProfileOptionValue,
  pub disable_loadscreen: ProfileOptionValue,
  pub disable_pin_browsers: ProfileOptionValue,

  pub custom_display_res: ProfileOptionValue,
  pub custom_homepage_url: ProfileOptionValue,
  pub custom_system_files: ProfileOptionValue,
}

impl Default for ProfileOptions {
  fn default() -> Self {
      Self {
        allow_external_laucher: false.into(),
        ignore_outdate: false.into(),
        disable_usb_manager: false.into(),
        disable_smart_iso: false.into(),
        unfold_ribbon: false.into(),
        reboot_default: false.into(),
        disable_recycle_bin: false.into(),
        auto_unattend: false.into(),
        drive_up_active: false.into(),
        drive_windows_first: false.into(),
        drive_order_another_way: false.into(),
        drive_mount_every_partition: false.into(),
        disable_loadscreen: false.into(),
        disable_pin_browsers: false.into(),
        custom_display_res: false.into(),
        custom_homepage_url: false.into(),
        custom_system_files: false.into(),
      }
  }
}

impl ProfileOptions {
  pub async fn parse(entry: &ProfileEntry) -> anyhow::Result<Self> {
    info!("start parse profile options with entry {:#?}", entry);
    let path = &entry.path;
    info!("new options with default");
    let mut options = Self::default();
    if !path.join(PATH_OPTIONS.clone()).exists() {
      return Ok(options);
    }

    if path.join(
      ProfileOption::AllowExternalLauncher.into_path()
    ).exists() && path.join(
      ProfileOption::AllowExternalLauncher.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::AllowExternalLauncher);
      options.allow_external_laucher = true.into();
    }

    if path.join(
      ProfileOption::AutoUnattend.into_path()
    ).exists() {
      info!("enabled option {:?}", ProfileOption::AutoUnattend);
      options.auto_unattend = true.into();
    }

    if path.join(
      ProfileOption::DisableLoadScreen.into_path()
    ).exists() && path.join(
      ProfileOption::DisableLoadScreen.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DisableLoadScreen); 
      options.disable_loadscreen = true.into();
    }

    if path.join(
      ProfileOption::DisablePinBrowsers.into_path()
    ).exists() && path.join(
      ProfileOption::DisablePinBrowsers.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DisablePinBrowsers);
      options.disable_pin_browsers = true.into();
    }

    if path.join(
      ProfileOption::DisableRecycleBin.into_path()
    ).exists() && path.join(
      ProfileOption::DisableRecycleBin.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DisableRecycleBin);
      options.disable_recycle_bin = true.into();
    }

    if path.join(
      ProfileOption::DisableSmartISO.into_path()
    ).exists() && path.join(
      ProfileOption::DisableSmartISO.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DisableSmartISO);
      options.disable_smart_iso = true.into();
    }

    if path.join(
      ProfileOption::DisableUSBManager.into_path()
    ).exists() && path.join(
      ProfileOption::DisableUSBManager.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DisableUSBManager);
      options.disable_usb_manager = true.into();
    }

    if path.join(
      ProfileOption::DriveMountEveryPartition.into_path()
    ).exists() && path.join(
      ProfileOption::DriveMountEveryPartition.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DriveMountEveryPartition);
      options.drive_mount_every_partition = true.into();
    }

    if path.join(
      ProfileOption::DriveOrderAnotherWay.into_path()
    ).exists() && path.join(
      ProfileOption::DriveOrderAnotherWay.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DriveOrderAnotherWay);
      options.drive_order_another_way = true.into();
    }

    if path.join(
      ProfileOption::DriveUpActive.into_path()
    ).exists() && path.join(
      ProfileOption::DriveUpActive.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DriveUpActive);
      options.drive_up_active = true.into();
    }

    if path.join(
      ProfileOption::DriveWindowsFirst.into_path()
    ).exists() && path.join(
      ProfileOption::DriveWindowsFirst.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::DriveWindowsFirst);
      options.drive_windows_first = true.into();
    }

    if path.join(
      ProfileOption::IgnoreOutdate.into_path()
    ).exists() && path.join(
      ProfileOption::IgnoreOutdate.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::IgnoreOutdate);
      options.ignore_outdate = true.into();
    }

    if path.join(
      ProfileOption::RebootDefault.into_path()
    ).exists() && path.join(
      ProfileOption::RebootDefault.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::RebootDefault);
      options.reboot_default = true.into();
    }

    if path.join(
      ProfileOption::UnfoldRibbon.into_path()
    ).exists() && path.join(
      ProfileOption::UnfoldRibbon.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::UnfoldRibbon);
      options.unfold_ribbon = true.into();
    }

    if path.join(
      ProfileOption::CustomSystemFilesFolder.into_path()
    ).exists() && path.join(
      ProfileOption::CustomSystemFilesFolder.into_path()
    ).is_dir() {
      info!("enabled option {:?}", ProfileOption::CustomSystemFilesFolder);
      options.custom_system_files = true.into();
    }

    // 兼容旧版
    if path.join(
      PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS.clone()
    ).exists() && path.join(
      PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS.clone()
    ).is_file() {
      warn!("the old-style custom display res config was deprecated! moved to new path.");
      fs::rename(
        path.join(PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS.clone()),
        path.join(ProfileOption::CustomDisplayRes.into_path()),
      ).await?;
    }

    if path.join(
      ProfileOption::CustomDisplayRes.into_path()
    ).exists() && path.join(
      ProfileOption::CustomDisplayRes.into_path()
    ).is_file() {
      info!("find custom display res config, try to parse");
      let original_text = fs::read_to_string(
        path.join(ProfileOption::CustomDisplayRes.into_path())
      ).await?;
      info!("read config file");

      info!("config text: {}", original_text);
      
      // 格式：宽(w) 高(h) 色位(b) 刷新率(f)
      let mut w: Option<usize> = Option::None;
      let mut h: Option<usize> = Option::None;
      let mut b: Option<usize> = Option::None;
      let mut f: Option<usize> = Option::None;
      
      for i in original_text.split(" ").map(|s| s.chars().collect::<Vec<_>>()) {
        if let Some(p) = i.get(0) {
          match p {
            &'w' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              info!("found `w` option, value = {}", n);
              w = Some(n);
            }
            &'h' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              info!("found `h` option, value = {}", n);
              h = Some(n);
            }
            &'b' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              info!("found `b` option, value = {}", n);
              b = Some(n);
            }
            &'f' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              info!("found `f` option, value = {}", n);
              f = Some(n);
            }
            _ => {
              info!("not found any option");
            }
          }
        }
      }

      match (w, h, b, f) {
        (
          Some(wv), 
          Some(hv), 
          Some(bv), 
          Some(fv)
        ) => {
          options.custom_display_res = ProfileOptionValue::CustomDisplayRes(wv, hv, bv, fv);
          info!("the custom display res config parsed, value = {:?}", options.custom_display_res);
        }
        _ => {
          warn!("the custom display res config is invaild");
        }
      }

    }

    if path.join(
      ProfileOption::CustomHomepageUrl.into_path()
    ).exists() && path.join(
      ProfileOption::CustomHomepageUrl.into_path()
    ).is_file() {
      info!("found the custom homepage url, try to parse");
      let original = fs::read_to_string(
        path.join(ProfileOption::CustomHomepageUrl.into_path())
      ).await?;
      info!("read file");
      info!("url text: {}", original);

      // 忽略大小写
      if original.to_lowercase() != "disable" {
        options.custom_homepage_url = ProfileOptionValue::CustomHomepageUrl(original);
        info!("new homepage url: {:?}", options.custom_homepage_url);
      } else {
        info!("the custom homepage option is disabled, use default");
      }
    }

    info!("profile options parsed, return");
    Ok(options)
  }
}

#[cfg(test)]
mod tests {

  use log::LevelFilter;

  fn init() {
    let _ = env_logger::builder()
      .filter_level(LevelFilter::Debug)
      .is_test(true)
      .try_init();
  }

  use crate::found::ProfileEntry;
  use super::ProfileOptions;
  use log::debug;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    init();
    if let Some(entry) = ProfileEntry::find_last().await? {
      let options = ProfileOptions::parse(&entry).await?;
      debug!("{:#?}", options);
    }

    Ok(())
  }
}