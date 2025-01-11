use crate::models::*;
use serde_json::Value;
use crate::MessageType;

fn determine_message_type(message: &Value) -> MessageType {
    if let Some(c) = message.get("type").and_then(|v| v.as_str()) {
        if c == "message-event" {
            if let Some(payload) = message.get("payload").and_then(|v| v.get("type")).and_then(|v| v.as_str()) {
                match payload {
                    "enqueued" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageEvent<Enqueued>>>(&message.to_string()) {
                            return MessageType::Enqueued(msg);
                        }
                    }
                    "failed" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageEvent<Failed>>>(&message.to_string()) {
                            return MessageType::Failed(msg);
                        }
                    }
                    "sent" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageEvent<Sent>>>(&message.to_string()) {
                            return MessageType::Sent(msg);
                        }
                    }
                    "delivered" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageEvent<Delivered>>>(&message.to_string()) {
                            return MessageType::Delivered(msg);
                        }
                    }
                    "read" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageEvent<Read>>>(&message.to_string()) {
                            return MessageType::Read(msg);
                        }
                    }
                    _ => {}
                }
            }
        } else if c == "message" {
            if let Some(payload) = message.get("payload").and_then(|v| v.get("type")).and_then(|v| v.as_str()) {
                match payload {
                    "text" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<Text>>>(&message.to_string()) {
                            return MessageType::Text(msg);
                        }
                    }
                    "image" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<Image>>>(&message.to_string()) {
                            return MessageType::Image(msg);
                        }
                    }
                    "file" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<File>>>(&message.to_string()) {
                            return MessageType::File(msg);
                        }
                    }
                    "audio" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<Audio>>>(&message.to_string()) {
                            return MessageType::Audio(msg);
                        }
                    }
                    "video" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<Video>>>(&message.to_string()) {
                            return MessageType::Video(msg);
                        }
                    }
                    "location" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<Location>>>(&message.to_string()) {
                            return MessageType::Location(msg);
                        }
                    }
                    "quick_reply" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<QuickReply>>>(&message.to_string()) {
                            return MessageType::QuickReply(msg);
                        }
                    }
                    "button_reply" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<ButtonReply>>>(&message.to_string()) {
                            return MessageType::ButtonReply(msg);
                        }
                    }
                    "list_reply" => {
                        if let Ok(msg) = serde_json::from_str::<ParentMessage<MessageGP<ListReply>>>(&message.to_string()) {
                            return MessageType::ListReply(msg);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    MessageType::Unknown
}





pub  fn init(message: &Value) -> MessageType {
    determine_message_type(message)
}
