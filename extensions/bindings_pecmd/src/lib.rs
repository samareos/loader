use std::path::PathBuf;
use anyhow::anyhow;
use tokio::process::{Child, Command};
use std::process::Stdio;
use log::{info, error};
use std::env;

pub struct Pecmd {
    exepath: PathBuf,
}

impl Pecmd {
    pub fn new(p: PathBuf) -> anyhow::Result<Self> {
        info!("create with {:?}", p);
        if !p.exists() {
            error!("not found bin file, at {:?}", p);
            return Err(anyhow!("not found bin file"));
        }

        Ok(Self {
            exepath: p,
        })
    }
}

impl Pecmd {
    pub async fn run(&self, options: &Vec<&str>) -> anyhow::Result<Child> {
        info!("run command, args = {:#?}", options);
        Ok(Command::new(&self.exepath)
            .args(options)
            .current_dir(env::current_dir()?)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
