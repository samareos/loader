use anyhow::anyhow;
use tokio::process::Child;
use std::path::PathBuf;
use tokio::process::Command;
use std::env;
use std::process;

use log::{info, error};

pub struct SevenZip {
  exe_path: PathBuf
}

impl SevenZip {
  pub fn new(exe: PathBuf) -> anyhow::Result<Self> {
    info!("create with {:?}", exe);
    if !exe.exists() {
      error!("not found bin file, at {:?}", exe);
      return Err(anyhow!("not found bin file"));
    }

    Ok(Self {
      exe_path: exe,
    })
  }

  pub async fn extract_all_files(&self, file: &str, out: &str) -> anyhow::Result<Child> {
    info!("extract all files with file = {:?}, out = {:?}", file, out);
    Ok(Command::new(&self.exe_path)
      .args(&["x", file, "-y", "-aos", &format!("-o\"{}\"", out)])
      .current_dir(env::current_dir()?)
      .stdout(process::Stdio::piped())
      .stderr(process::Stdio::piped())
      .stdin(process::Stdio::piped())
      .spawn()?)
  }

  pub async fn run(&self, args: &Vec<&str>) -> anyhow::Result<Child> {
    info!("run command, args = {:#?}", args);
    Ok(Command::new(&self.exe_path)
      .args(args)
      .current_dir(env::current_dir()?)
      .stdout(process::Stdio::piped())
      .stderr(process::Stdio::piped())
      .stdin(process::Stdio::piped())
      .spawn()?)
  }
}

