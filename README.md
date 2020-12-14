# EventSource-Threaded
EventSource-Threaded is a Rust library for reading from Server-Sent Events endpoints. It transparently
sends HTTP requests on a separate thread and only exposes a stream of events to the user via a channel receiver. It handles automatic
reconnection and parsing of the `text/event-stream` data format.

EventSource-Threaded is a fork of [this EventSource library](https://github.com/lluchs/eventsource) created by [lluchs](https://github.com/lluchs). The primary differences are:
- The EventSource client is now threaded
- Adding arbitrary header values is now supported (important for use cases where you need to set cookies for Authorization for example)
- Minor renaming/removing methods/interface change

## Examples
```rust
use eventsource-threaded::reqwest::Client;
use reqwest::Url;

fn main() {
    let receiver = EventSource::new(Url::parse("http://example.com").unwrap());
    loop {
        println!("Received Event: {:?}", receiver.recv());
    }

}
```