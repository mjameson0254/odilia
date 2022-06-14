pub mod config;
pub use config::Plugin;

use std::{
    io,
    os::unix::io::{AsRawFd, FromRawFd},
    process::Stdio,
};

use tokio::net::UnixStream;

pub fn spawn(plugin: Plugin) -> io::Result<()> {
    let (a, b) = UnixStream::pair()?;
    let b = b.as_raw_fd();

    let mut cmd = plugin.into_command();
    // Safety: This is always a valid file descriptor
    cmd.stdin(unsafe { Stdio::from_raw_fd(b) })
        .stdout(unsafe { Stdio::from_raw_fd(b) });

    let mut child = cmd.spawn()?;
    tokio::spawn(async move {
        match child.wait().await {
            Ok(status) => tracing::info!(%status, "Plugin exited"),
            Err(e) => tracing::error!(error = %e, "Failed running plugin"),
        }
    });
    Ok(())
}
