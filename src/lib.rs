mod a11y;
mod args;
mod logging;
use a11y::{BusProxy, StatusProxy};
use atspi_sys::registry::RegistryProxy;
use color_eyre::eyre::{Result, WrapErr};

use std::{process::exit, str::FromStr, sync::Arc};
use tracing::{
    debug,
    //error,
    info,
};
use zbus::{export::futures_util::StreamExt, Address, Connection, ConnectionBuilder};
#[tracing::instrument]
pub fn setup() {
    logging::init();
    info!("odilia screenreader successfully started!");
    color_eyre::install().unwrap();
    let _args = args::parse();
}
#[tracing::instrument]
pub fn cleanup() {
    info!("odilia screenreader is shutting down");
    exit(0);
}
pub async fn init_accessibility<'a>() -> Result<(RegistryProxy<'a>, BusProxy<'a>)> {
    let connection = Connection::session()
        .await
        .wrap_err("unable to connect to dbus session")?;
    let bproxy = BusProxy::new(&connection).await.wrap_err(
        "error while creating a proxy to the session buss entrypoint to the atspi registrid",
    )?;
    let sproxy = StatusProxy::new(&connection).await.wrap_err("unable to create status proxy to the dbus interface used to get and set screen reader status")?;
    let addr = bproxy
        .get_address()
        .await
        .wrap_err("error while getting the address of the atspi registry on the system bus")?;
    debug!("Found the a11y bus {}!", addr);
    let rstream =
        Address::from_str(&addr).wrap_err("can't convert the address into a dbus stream")?;
    let rconnection = ConnectionBuilder::address(rstream)?
        .build()
        .await
        .wrap_err("error while creating a connection to the dbus registry")?;
    let rproxy = RegistryProxy::new(&rconnection)
        .await
        .wrap_err("unable to create atspi registry proxy from connection")?;

    sproxy.set_screen_reader_enabled(true).await.wrap_err(
        "error setting the screen_reader_enabled property on the status dbus interface",
    )?;
    sproxy.set_is_enabled(true).await.wrap_err(
        "error setting the screen_reader_enabled property on the status dbus interface",
    )?;
    let _screen_reader_enabled = {
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

    Ok((rproxy, bproxy))
}
pub async fn register_events(rproxy: Arc<RegistryProxy<'_>>) -> Result<()> {
    rproxy
        .register_event("object:state-changed:focused")
        .await?;
    rproxy.register_event("object:text-caret-moved").await?;
    Ok(())
}
pub async fn spawn_event_tasks(rproxy: Arc<RegistryProxy<'static>>, bproxy: Arc<BusProxy<'static>>) -> Result<()> {
    tokio::task::spawn(process_events(Arc::clone(&rproxy)))
        .await?
        .wrap_err("error while processing accessibility events")?;
    tokio::task::spawn(process_signals(bproxy.clone()))
        .await?
        .wrap_err("error while processing dbus sygnals")?;
    Ok(())
}
#[tracing::instrument(skip(rproxy))]
pub async fn process_events(rproxy: Arc<RegistryProxy<'_>>) -> Result<(), zbus::Error> {
    let events = rproxy.get_registered_events().await?;
    tracing::debug!("Events registerd: {}", events.len());
    for (e1, e2) in events {
        tracing::debug!("Event {},{} is registered", e1, e2);
    }
    Ok(())
}
#[tracing::instrument(skip(proxy))]
pub async fn process_signals(proxy: Arc<BusProxy<'_>>) -> Result<()> {
    let mut stream = proxy.receive_all_signals().await?;
    while let Some(_e) = stream.next().await {
        debug!("Event received!");
    }
    Ok(())
}
