//! # EventSource-Threaded
//!
//! EventSource-Threaded is a Rust library for reading from Server-Sent Events endpoints. It transparently
//! sends HTTP requests on a separate thread and only exposes a stream of events to the user via a channel receiver. It handles automatic
//! reconnection and parsing of the `text/event-stream` data format.
//!
//! # Examples
//!
//! ```no_run
//! use eventsource-threaded::reqwest::Client;
//! use reqwest::Url;
//!
//! fn main() {
//!     let receiver = EventSource::new(Url::parse("http://example.com").unwrap());
//!     loop {
//!         println!("Received Event: {:?}", receiver.recv());
//!     }

//! }
//! ```
//!

// Generic text/event-stream parsing and serialization.
pub mod event;
// HTTP interface
#[cfg(feature = "with-reqwest")]
pub mod eventsource;
#[cfg(feature = "with-reqwest")]
mod reqwest;

pub use eventsource::{EventSource, ReceiverSource};
