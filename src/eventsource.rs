use super::event::Event;
use crate::reqwest::{Client, Result};
use crossbeam::channel::{unbounded, Receiver};
use reqwest::header::HeaderMap;
use std::thread;

/// The type of the `Receiver` which sends back event from the EventSource
/// client.
pub type ReceiverSource = Receiver<Result<Event>>;

pub struct EventSource {}

impl EventSource {
    /// Constructs a new EventSource client for the given URL on
    /// a separate thread and sends all events through a channel
    /// through a `Receiver`.
    ///
    pub fn new(url: reqwest::Url, headers: HeaderMap) -> ReceiverSource {
        let (s, r) = unbounded();

        thread::spawn(move || {
            let client = Client::new(url, headers);
            for event in client.into_iter() {
                let _ = s.send(event);
            }
        });
        r
    }
}
