mod a11y;
mod args;

use a11y::{BusProxy, StatusProxy};
use atspi_sys::{
    registry::RegistryProxy,
    event::EventObjectProxy,
};
use color_eyre::eyre::{Result, WrapErr};
use logging;

use std::{process::exit, str::FromStr, sync::Arc};
use tracing::{
    debug,
    //error,
    info,
};
use zbus::{
    export::futures_util::StreamExt,
    Address,
    Connection,
    ConnectionBuilder,
};

#[tracing::instrument]
pub async fn setup() {
    logging::init();
    info!("odilia screenreader successfully started!");
    color_eyre::install().unwrap();
    let _args = args::parse();
    //for some reason, this doesn't work, the executor keeps blocking on that future and it only returns back to the caller when an actual sigint is recieved, even though I used the task API to hopefully start the signal listener loop in paralell. Committing this knowing full well I'll have to come back to it later, but most of the pieces are there
    //register_keyboard_interrupt().await;
}
#[tracing::instrument]
async fn register_keyboard_interrupt() {
    debug!("registering sigint handler");
    use tokio::signal;

    tokio::task::spawn(async {
        signal::ctrl_c()
            .await
            .expect("unable to register sigint handler");
    })
    .await
    .expect("something went wrong while executing the future");
    debug!("sigint handler successfully registered");
    cleanup();
}
#[tracing::instrument]
pub fn cleanup() {
    info!("odilia screenreader is shutting down");
    exit(0);
}
pub async fn init_accessibility<'a>() -> Result<(RegistryProxy<'a>, BusProxy<'a>, EventObjectProxy<'a>)> {
    let connection = Connection::session()
        .await
        .wrap_err("unable to connect to dbus session")?;
    let dbproxy = DBusProxy::new(&connection).await.wrap_err("error connecting to DBus proxy")?;
    let ans = dbproxy.add_match("type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved',path='/org/a11y/atspi/registry'").await?;
    debug!("{:?}", ans);
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
    let eproxy = EventObjectProxy::new(&rconnection)
        .await
        .wrap_err("unable to create proxy for atspi.Event.Object")?;
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

    Ok((rproxy, bproxy, eproxy))
}
pub async fn register_events(rproxy: Arc<RegistryProxy<'_>>) -> Result<()> {
    rproxy
        .register_event("object:state-changed:focused")
        .await?;
    rproxy.register_event("object:text-caret-moved").await?;
    rproxy.register_event("focus:").await?;
    rproxy.register_event("document:load-complete").await?;
    rproxy.register_event("object:text-changed:delete").await?;
    Ok(())
}
pub async fn spawn_event_tasks(
    rproxy: Arc<RegistryProxy<'static>>,
    bproxy: Arc<BusProxy<'static>>,
    eproxy: Arc<EventObjectProxy<'static>>
) -> Result<()> {
    tokio::task::spawn(process_events(Arc::clone(&rproxy)))
        .await?
        .wrap_err("error while processing accessibility events")?;
    tokio::task::spawn(process_signals(eproxy.clone()))
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
pub async fn process_signals(proxy: Arc<EventObjectProxy<'_>>) -> Result<()> {
    let mut stream = proxy.receive_all_signals().await?;
    while let Some(_e) = stream.next().await {
        debug!("Event received!");
    }
    Ok(())
}
