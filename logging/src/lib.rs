//!Logging with the [`tracing`] crate.
//!
//! Not much here yet, but this will get more complex if we decide to add other layers for error
//! reporting, tokio-console, etc.

mod tracing_tree_layer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

/// Initialise the logging stack. Right now this just sets the RUST_LOG environment variable based on the current build(debug or release), then calls [`tracing_subscriber::fmt::init`].
pub fn init() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(tracing_tree_layer::init_layer())
        .init();
}
