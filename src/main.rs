mod a11y;
mod args;
mod logging;
use a11y::{BusProxy, StatusProxy};
use atspi::registry::RegistryProxy;
use std::{error::Error, str::FromStr};
use tracing::{
    debug,
    //error,
    info,
};
use zbus::{
    dbus_proxy, export::futures_util::StreamExt, Address, Connection, ConnectionBuilder,
    Result as DBusResult,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::init();
    let _args = args::parse();
    let connection = Connection::session().await?;
    let bproxy = BusProxy::new(&connection).await?;
    let sproxy = StatusProxy::new(&connection).await?;
    let addr = bproxy.get_address().await?;
    let rstream = Address::from_str(&addr)?;
    let rconnection = ConnectionBuilder::address(rstream)?.build().await?;
    let rproxy = RegistryProxy::new(&rconnection).await?;
    // NOTE: this doesn't work for some reason? Always shows false for me (Tait)

    sproxy.set_screen_reader_enabled(true).await?;
    sproxy.set_is_enabled(true).await?;

    info!("Hello, world!");
    debug!("Found the a11y bus {}!", addr);
    let screen_reader_enabled = {
        if sproxy.screen_reader_enabled().await? {
            debug!(
                "screen reader state has been successfully set to on in the accessibility system"
            );
            true
        } else {
            debug!("failed to set screen reader property to on in the accessibility system");
            false
        }
    };
    rproxy
        .register_event("object:state-changed:focused")
        .await?;
    rproxy.register_event("object:text-caret-moved").await?;

    let events = rproxy.get_registered_events().await?;
    tracing::debug!("Events registerd: {}", events.len());
    for (e1, e2) in events {
        tracing::debug!("Event {},{} is registered", e1, e2);
    }
    let mut stream = bproxy.receive_all_signals().await?;
    while let Some(_e) = stream.next().await {
        debug!("Event received!");
    }
    Ok(())
}
