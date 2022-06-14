use std::{collections::HashMap, ffi::OsString};

use serde::Deserialize;
use tokio::process::Command;

#[derive(Deserialize)]
pub struct Plugin {
    #[serde(default = "returns_true")]
    pub enabled: bool,
    pub command: Vec<OsString>,
}

impl Plugin {
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(&self.command[0]);
        cmd.args(&self.command[1..]);
        cmd
    }
}

#[derive(Deserialize)]
pub struct Plugins {
    #[serde(flatten)]
    pub plugins: HashMap<String, Plugin>,
}

fn returns_true() -> bool {
    true
}
