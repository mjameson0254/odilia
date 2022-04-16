use color_eyre::eyre::Result;
use odilia::*;
use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<()> {
    setup().await;
    let (rproxy, bproxy) = init_accessibility().await?;
    let rproxy = Arc::new(rproxy);
    let bproxy = Arc::new(bproxy);
    register_events(Arc::clone(&rproxy)).await?;

    spawn_event_tasks(Arc::clone(&rproxy), Arc::clone(&bproxy)).await?;
    cleanup();
    Ok(())
}
