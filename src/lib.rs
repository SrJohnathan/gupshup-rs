use crate::models::{
    Audio, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location,
    MessageEvent, MessageGP, ParentMessage, QuickReply, Read, Sent, Text, Video,
};
use serde_json::Value;

mod core;
pub mod models;

#[derive(Debug)]
pub enum MessageType {
    Enqueued(ParentMessage<MessageEvent<Enqueued>>),
    Failed(ParentMessage<MessageEvent<Failed>>),
    Sent(ParentMessage<MessageEvent<Sent>>),
    Delivered(ParentMessage<MessageEvent<Delivered>>),
    Read(ParentMessage<MessageEvent<Read>>),
    Text(ParentMessage<MessageGP<Text>>),
    Image(ParentMessage<MessageGP<Image>>),
    File(ParentMessage<MessageGP<File>>),
    Audio(ParentMessage<MessageGP<Audio>>),
    Video(ParentMessage<MessageGP<Video>>),
    Location(ParentMessage<MessageGP<Location>>),
    QuickReply(ParentMessage<MessageGP<QuickReply>>),
    ButtonReply(ParentMessage<MessageGP<ButtonReply>>),
    ListReply(ParentMessage<MessageGP<ListReply>>),
    Unknown,
}

pub fn deserialize(value: &Value) -> MessageType {
    core::init(value)
}

/*pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
