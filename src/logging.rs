//!Logging with the [`tracing`] crate.
//!
//! Not much here yet, but this will get more complex if we decide to add other layers for error
//! reporting, tokio-console, etc.

/// Initialise the logging stack. Right now this just sets the RUST_LOG environment variable based on the current build(debug or release), then calls [`tracing_subscriber::fmt::init`].
pub fn init() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
}
