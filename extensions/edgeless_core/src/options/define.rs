use std::path::PathBuf;
use crate::found::ProfileEntry;
use tokio::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  pub static ref PATH_OPTIONS: PathBuf = PathBuf::from("Config");

  pub static ref PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS: PathBuf = PathBuf::from("分辨率.txt");

  pub static ref PATH_CUSTOM_DISPLAY_RES: PathBuf = PATH_OPTIONS.join("分辨率.txt");
  pub static ref PATH_CUSTOM_HOME_PAGE: PathBuf = PATH_OPTIONS.join("HomePage.txt");

  pub static ref PATH_CUSTOM_SYSTEM_FILES_FOLDER: PathBuf = PathBuf::from("Windows");

  pub static ref EXT_CUSTOM_SYSTEM_SETUP_IMAGES: Regex = Regex::new("^iso|wim|esd$").unwrap();
  pub static ref PATH_CUSTOM_SYSTEM_SETUP_IMAGES_FOLDER: PathBuf = PathBuf::from("System"); // ON ANY DISK
  
  pub static ref PATH_CUSTOM_WALLPAPER: PathBuf = PathBuf::from("wp.jpg");
  pub static ref PATH_CUSTOM_WALLPAPER_BAK: PathBuf = PathBuf::from("wp_backup.jpg");
  pub static ref PATH_EXTERNAL_LAUNCHER: PathBuf = PathBuf::from("Launcher.cmd");

  pub static ref EXT_PLUGIN_PATTERN: Regex = Regex::new("^7z.*$").unwrap();
  pub static ref EXT_PLUGIN_NORMAL: &'static str = "7z";
  pub static ref EXT_PLUGIN_DISABLE: &'static str = "7zf";
  pub static ref EXT_PLUGIN_LOCALBOOST: &'static str = "7zl";
  pub static ref EXT_PLUGIN_DOTNET: &'static str = "7zn";
  pub static ref PATH_PLUGIN_RESOURCES: PathBuf = PathBuf::from("Resource");

  pub static ref EXT_THEME_PACK: &'static str = "eth";
  pub static ref EXT_THEME_ICON: &'static str = "eis";
  pub static ref EXT_THEME_CURSOR_STYLE: &'static str = "ems";
  pub static ref EXT_THEME_SIB_CONFIG: &'static str = "esc";
  pub static ref EXT_THEME_SYS_ICON: &'static str = "ess";
  pub static ref EXT_THEME_LOADSCREEN: &'static str = "els";
  pub static ref PATH_THEME_PACK_WALLPAPAER: PathBuf = PathBuf::from("WallPaper.jpg");
  pub static ref PATH_THEME_PACK_INTRO_FOLDER: PathBuf = PathBuf::from("Intro");
  pub static ref PATH_THEME_PACK_INTRO_TEXT: PathBuf = PathBuf::from("Intro.txt");
  pub static ref PATH_THEME_PACK_INTRO_SCRIPT: PathBuf = PathBuf::from("Intro.wcs");

  pub static ref PATH_THEME_DEFAULT: PathBuf = PathBuf::from("Default");
  pub static ref PATH_THEME_DEFAULT_ICON: PathBuf = PATH_THEME_DEFAULT.join("IconPack.eis");
  pub static ref PATH_THEME_DEFAULT_CURSOR_STYLE: PathBuf = PATH_THEME_DEFAULT.join("MouseStyle.ems");
  pub static ref PATH_THEME_DEFAULT_SIB_CONFIG: PathBuf = PATH_THEME_DEFAULT.join("StartIsBackConfig.esc");
  pub static ref PATH_THEME_DEFAULT_SYS_ICON: PathBuf = PATH_THEME_DEFAULT.join("SystemIconPack.ess");
  pub static ref PATH_THEME_DEFAULT_LOCALSCREEN_FOLDER: PathBuf = PATH_THEME_DEFAULT.join("LocalScreen");
  pub static ref PATH_THEME_DEFAULT_INTRO_FOLDER: PathBuf = PATH_THEME_DEFAULT.join("Intro");
  pub static ref PATH_THEME_DEFAULT_INTRO_TEXT: PathBuf = PathBuf::from("Intro.txt");
  pub static ref PATH_THEME_DEFAULT_INTRO_SCRIPT: PathBuf = PathBuf::from("Intro.wcs");

  pub static ref PATH_OPTION_ALLOW_EXTERNAL_LAUNCHER: PathBuf = PATH_OPTIONS.join("Developer");
  pub static ref PATH_OPTION_IGNORE_OUTDATE: PathBuf = PATH_OPTIONS.join("NoOutDateCheck");
  pub static ref PATH_OPTION_DISABLE_USB_MANAGER: PathBuf = PATH_OPTIONS.join("DisableUSBManager");
  pub static ref PATH_OPTION_DISABLE_SMART_ISO: PathBuf = PATH_OPTIONS.join("DisableSmartISO");
  pub static ref PATH_OPTION_UNFOLD_RIBBON: PathBuf = PATH_OPTIONS.join("UnfoldRibbon");
  pub static ref PATH_OPTION_REBOOT_DEFAULT: PathBuf = PATH_OPTIONS.join("RebootDefault");
  pub static ref PATH_OPTION_DISABLE_RECYCLE_BIN: PathBuf = PATH_OPTIONS.join("DisableRecycleBin");
  pub static ref PATH_OPTION_AUTO_UNATTEND: PathBuf = PATH_OPTIONS.join("AutoUnattend");
  pub static ref PATH_OPTION_DRV_UP_ACT: PathBuf = PATH_OPTIONS.join("UpActDrv");
  pub static ref PATH_OPTION_DRV_WIN_FIRST: PathBuf = PATH_OPTIONS.join("WinFirst");
  pub static ref PATH_OPTION_DRV_ORDER_ANOTHER: PathBuf = PATH_OPTIONS.join("OrderDrvAnotherWay");
  pub static ref PATH_OPTION_DRV_MOUNT_EVERY_PART: PathBuf = PATH_OPTIONS.join("MountEveryPartition");
  pub static ref PATH_OPTION_DISABLE_LOADSCREEN: PathBuf = PATH_OPTIONS.join("DisableLoadScreen");
  pub static ref PATH_OPTION_DISABLE_PIN_BROWSERS: PathBuf = PATH_OPTIONS.join("DisablePinBrowsers");
  
}

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


#[derive(Debug, Clone, Copy)]
pub enum ProfileOption {
  CustomDisplayRes, // (usize, usize, usize, usize),
  CustomHomepageUrl, // (String, bool),
  CustomSystemFilesFolder, // = CUSTOM_SYSTEM_FILES_FOLDER,

  AllowExternalLauncher, // = PATH_OPTION_ALLOW_EXTERNAL_LAUNCHER,
  IgnoreOutdate, // = PATH_OPTION_IGNORE_OUTDATE,
  DisableUSBManager, // = PATH_OPTION_DISABLE_USB_MANAGER,
  DisableSmartISO, // = PATH_OPTION_DISABLE_SMART_ISO,
  UnfoldRibbon, // = PATH_OPTION_UNFOLD_RIBBON,
  RebootDefault, // = PATH_OPTION_REBOOT_DEFAULT,
  DisableRecycleBin, // = PATH_OPTION_DISABLE_RECYCLE_BIN,
  AutoUnattend, // = PATH_OPTION_AUTO_UNATTEND,
  DriveUpActive, // = PATH_OPTION_DRV_UP_ACT,
  DriveWindowsFirst, // = PATH_OPTION_DRV_WIN_FIRST,
  DriveOrderAnotherWay, // = PATH_OPTION_DRV_ORDER_ANOTHER,
  DriveMountEveryPartition, // = PATH_OPTION_DRV_MOUNT_EVERY_PART,
  DisableLoadScreen, // = PATH_OPTION_DISABLE_LOADSCREEN,
  DisablePinBrowsers, // = PATH_OPTION_DISABLE_PIN_BROWSERS,
}

impl ProfileOption {
  fn into_path(self) -> PathBuf {
    match self {
        ProfileOption::CustomDisplayRes => PATH_CUSTOM_DISPLAY_RES.clone(),
        ProfileOption::CustomHomepageUrl => PATH_CUSTOM_HOME_PAGE.clone(),
        ProfileOption::AllowExternalLauncher => PATH_OPTION_ALLOW_EXTERNAL_LAUNCHER.clone(),
        ProfileOption::IgnoreOutdate => PATH_OPTION_IGNORE_OUTDATE.clone(),
        ProfileOption::DisableUSBManager => PATH_OPTION_DISABLE_USB_MANAGER.clone(),
        ProfileOption::DisableSmartISO => PATH_OPTION_DISABLE_SMART_ISO.clone(),
        ProfileOption::UnfoldRibbon => PATH_OPTION_UNFOLD_RIBBON.clone(),
        ProfileOption::RebootDefault => PATH_OPTION_REBOOT_DEFAULT.clone(),
        ProfileOption::DisableRecycleBin => PATH_OPTION_DISABLE_RECYCLE_BIN.clone(),
        ProfileOption::AutoUnattend => PATH_OPTION_AUTO_UNATTEND.clone(),
        ProfileOption::DriveUpActive => PATH_OPTION_DRV_UP_ACT.clone(),
        ProfileOption::DriveWindowsFirst => PATH_OPTION_DRV_WIN_FIRST.clone(),
        ProfileOption::DriveOrderAnotherWay => PATH_OPTION_DRV_ORDER_ANOTHER.clone(),
        ProfileOption::DriveMountEveryPartition => PATH_OPTION_DRV_MOUNT_EVERY_PART.clone(),
        ProfileOption::DisableLoadScreen => PATH_OPTION_DISABLE_LOADSCREEN.clone(),
        ProfileOption::DisablePinBrowsers => PATH_OPTION_DISABLE_PIN_BROWSERS.clone(),
        ProfileOption::CustomSystemFilesFolder => PATH_CUSTOM_SYSTEM_FILES_FOLDER.clone(),
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
    let path = &entry.path;
    let mut options = Self::default();
    if !path.join(PATH_OPTIONS.clone()).exists() {
      return Ok(options);
    }

    if path.join(
      ProfileOption::AllowExternalLauncher.into_path()
    ).exists() && path.join(
      ProfileOption::AllowExternalLauncher.into_path()
    ).is_dir() {
      options.allow_external_laucher = true.into();
    }

    if path.join(
      ProfileOption::AutoUnattend.into_path()
    ).exists() {
      options.auto_unattend = true.into();
    }

    if path.join(
      ProfileOption::DisableLoadScreen.into_path()
    ).exists() && path.join(
      ProfileOption::DisableLoadScreen.into_path()
    ).is_dir() {
      options.disable_loadscreen = true.into();
    }

    if path.join(
      ProfileOption::DisablePinBrowsers.into_path()
    ).exists() && path.join(
      ProfileOption::DisablePinBrowsers.into_path()
    ).is_dir() {
      options.disable_pin_browsers = true.into();
    }

    if path.join(
      ProfileOption::DisableRecycleBin.into_path()
    ).exists() && path.join(
      ProfileOption::DisableRecycleBin.into_path()
    ).is_dir() {
      options.disable_recycle_bin = true.into();
    }

    if path.join(
      ProfileOption::DisableSmartISO.into_path()
    ).exists() && path.join(
      ProfileOption::DisableSmartISO.into_path()
    ).is_dir() {
      options.disable_smart_iso = true.into();
    }

    if path.join(
      ProfileOption::DisableUSBManager.into_path()
    ).exists() && path.join(
      ProfileOption::DisableUSBManager.into_path()
    ).is_dir() {
      options.disable_usb_manager = true.into();
    }

    if path.join(
      ProfileOption::DriveMountEveryPartition.into_path()
    ).exists() && path.join(
      ProfileOption::DriveMountEveryPartition.into_path()
    ).is_dir() {
      options.drive_mount_every_partition = true.into();
    }

    if path.join(
      ProfileOption::DriveOrderAnotherWay.into_path()
    ).exists() && path.join(
      ProfileOption::DriveOrderAnotherWay.into_path()
    ).is_dir() {
      options.drive_order_another_way = true.into();
    }

    if path.join(
      ProfileOption::DriveUpActive.into_path()
    ).exists() && path.join(
      ProfileOption::DriveUpActive.into_path()
    ).is_dir() {
      options.drive_up_active = true.into();
    }

    if path.join(
      ProfileOption::DriveWindowsFirst.into_path()
    ).exists() && path.join(
      ProfileOption::DriveWindowsFirst.into_path()
    ).is_dir() {
      options.drive_windows_first = true.into();
    }

    if path.join(
      ProfileOption::IgnoreOutdate.into_path()
    ).exists() && path.join(
      ProfileOption::IgnoreOutdate.into_path()
    ).is_dir() {
      options.ignore_outdate = true.into();
    }

    if path.join(
      ProfileOption::RebootDefault.into_path()
    ).exists() && path.join(
      ProfileOption::RebootDefault.into_path()
    ).is_dir() {
      options.reboot_default = true.into();
    }

    if path.join(
      ProfileOption::UnfoldRibbon.into_path()
    ).exists() && path.join(
      ProfileOption::UnfoldRibbon.into_path()
    ).is_dir() {
      options.unfold_ribbon = true.into();
    }

    if path.join(
      ProfileOption::CustomSystemFilesFolder.into_path()
    ).exists() && path.join(
      ProfileOption::CustomSystemFilesFolder.into_path()
    ).is_dir() {
      options.custom_system_files = true.into();
    }

    // 兼容旧版
    if path.join(
      PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS.clone()
    ).exists() && path.join(
      PATH_OLD_CUSTOM_DISPLAY_RES_OPTIONS.clone()
    ).is_file() {
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
      let original_text = fs::read_to_string(
        path.join(ProfileOption::CustomDisplayRes.into_path())
      ).await?;
      
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
              w = Some(n);
            }
            &'h' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              h = Some(n);
            }
            &'b' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              b = Some(n);
            }
            &'f' => {
              let n = i.iter().skip(1).collect::<String>().parse()?;
              f = Some(n);
            }
            _ => {}
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
        }
        _ => {}
      }

    }

    if path.join(
      ProfileOption::CustomHomepageUrl.into_path()
    ).exists() && path.join(
      ProfileOption::CustomHomepageUrl.into_path()
    ).is_file() {
      let original = fs::read_to_string(
        path.join(ProfileOption::CustomHomepageUrl.into_path())
      ).await?;

      // 忽略大小写
      if original.to_lowercase() != "disable" {
        options.custom_homepage_url = ProfileOptionValue::CustomHomepageUrl(original);
      }
    }
    Ok(options)
  }
}

#[cfg(test)]
mod tests {
    use crate::found::ProfileEntry;

    use super::ProfileOptions;

  #[tokio::test]
  async fn it_works() -> anyhow::Result<()> {
    if let Some(entry) = ProfileEntry::find_last().await? {
      let options = ProfileOptions::parse(&entry).await?;
      println!("{:#?}", options);
    }

    Ok(())
  }
}