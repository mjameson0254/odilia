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
    pub fn into_command(mut self) -> Command {
        let exe = self.command.remove(0);
        let mut cmd = Command::new(exe);
        cmd.args(self.command);
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
