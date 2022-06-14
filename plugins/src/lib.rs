pub mod config;
pub use config::Plugin;

use std::{
    io,
    os::unix::io::{AsRawFd, FromRawFd},
    process::Stdio,
};

use tokio::{net::UnixStream, process::Child};

pub fn spawn(name: String, plugin: &Plugin) -> io::Result<()> {
    let (a, b) = UnixStream::pair()?;
    let b = b.as_raw_fd();

    let mut cmd = plugin.command();
    // Safety: This is always a valid file descriptor
    cmd.stdin(unsafe { Stdio::from_raw_fd(b) })
        .stdout(unsafe { Stdio::from_raw_fd(b) })
        .kill_on_drop(true);

    let child = cmd.spawn()?;
    tokio::spawn(wait_on_child(name, child));
    Ok(())
}

#[tracing::instrument(skip(child))]
async fn wait_on_child(name: String, mut child: Child) {
    match child.wait().await {
        Ok(status) => tracing::info!(%status, "Plugin exited"),
        Err(e) => tracing::error!(error = %e, "Failed running plugin"),
    }
}
