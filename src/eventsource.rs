use super::event::Event;
use crate::reqwest::{Client, Result};
use crossbeam::channel::{unbounded, Receiver};
use reqwest::header::HeaderMap;
use std::thread;

pub struct EventSource {}

impl EventSource {
    /// Constructs a new EventSource client for the given URL on
    /// a separate thread and sends all events through a channel
    /// through a `Receiver`.
    ///
    pub fn new(url: reqwest::Url, headers: HeaderMap) -> Receiver<Result<Event>> {
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
