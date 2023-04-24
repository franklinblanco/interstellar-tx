mod itx;
mod queue;
mod macros;
mod headers;
/// ### This part of the code needs the feature `actix` to be enabled.
/// This is where all the routes that will be placed into the actix AppRouter. You can manually implement them, but these are convenience wrappers.
#[cfg(feature = "actix")]
pub mod routes;
pub mod error;

pub use queue::add_to_queue;

/// This is the key for the response header to be sent back to the main service (the proxy)
pub const RESPONSE_HEADER_UUID_KEY: &str = "X-ITX-UUID";