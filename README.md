# gupshup-rs

`gupshup-rs` is a Rust library designed to deserialize data received from Gupshup webhooks related to WhatsApp. It provides a structured way to efficiently process events and messages.

## Features

- Easily deserialize payloads from Gupshup webhooks.
- Support for multiple message types, including text, image, audio, video, and more.
- Integration with WhatsApp events such as delivered, read, sent messages, etc.

## Installation

Add `gupshup-rs` to your `Cargo.toml` file:

```toml
[dependencies]
gupshup-rs = "0.0.1"
```

git:

```toml
[dependencies]
gupshup-rs = { git = "https://github.com/SrJohnathan/gupshup-rs.git" }
```


## Usage Examples

### Webhook Handler

Here is an example of how to use the library to process messages directly in an asynchronous function, such as part of a server:

```rust
pub async fn web_hook(task: serde_json::Value) {
    match gupshup_rs::deserialize(&task) {
        MessageType::Enqueued(x) => {
            println!("Queued message: {:?}", x);
        }
        MessageType::Failed(x) => {
            println!("Failed message: {:?}", x);
        }
        MessageType::Sent(x) => {
            println!("Sent message: {:?}", x);
        }
        MessageType::Delivered(x) => {
            println!("Delivered message: {:?}", x);
        }
        MessageType::Read(x) => {
            println!("Read message: {:?}", x);
        }
        MessageType::Text(x) => {
            println!("Text message: {:?}", x);
        }
        MessageType::Image(x) => {
            println!("Image message: {:?}", x);
        }
        MessageType::File(x) => {
            println!("File message: {:?}", x);
        }
        MessageType::Audio(x) => {
            println!("Audio message: {:?}", x);
        }
        MessageType::Video(x) => {
            println!("Video message: {:?}", x);
        }
        MessageType::Location(x) => {
            println!("Location message: {:?}", x);
        }
        MessageType::QuickReply(x) => {
            println!("Quick reply: {:?}", x);
        }
        MessageType::ButtonReply(x) => {
            println!("Button reply: {:?}", x);
        }
        MessageType::ListReply(x) => {
            println!("List reply: {:?}", x);
        }
        MessageType::Unknown => {
            println!("Unknown message type");
        }
    }
}
```

### Supported Features

#### Message Types
- Text (`text`)
- Image (`image`)
- Audio (`audio`)
- Video (`video`)
- Location (`location`)
- Quick reply (`quick_reply`)
- Button reply (`button_reply`)
- List reply (`list_reply`)

#### WhatsApp Events
- Sent message (`sent`)
- Delivered message (`delivered`)
- Read message (`read`)
- Queued message (`enqueued`)
- Failed message (`failed`)

## Contributing

Contributions are welcome! Follow the steps below to collaborate:

1. Fork the repository.
2. Create a branch for your feature/fix: `git checkout -b my-feature`.
3. Make your changes and commit them: `git commit -m 'Add my feature'`.
4. Push your branch: `git push origin my-feature`.
5. Open a Pull Request.

## License

This project is licensed under the MIT and Apache 2.0 licenses. See the `LICENSE-MIT` and `LICENSE-APACHE` files for more details.

