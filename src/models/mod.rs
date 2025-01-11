
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct MessageGupshup {
    pub r#type: String,
    pub text: Option<String>,
    pub originalUrl: Option<String>,
    pub previewUrl: Option<String>,
    pub url: Option<String>,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ParentMessage<T> {
    pub app: String,
    pub timestamp: isize,
    pub version: i32,
    pub r#type: String,
    pub payload: T,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Read {
    pub ts: i32,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Sent {
    pub ts: i32,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Delivered {
    pub ts: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Enqueued {
    pub whatsappMessageId: String,
    pub r#type: String,

}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Failed {
    pub code: isize,
    pub reason: String,

}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MessageEvent<T> {
    pub id: String,
    pub gsId: Option<String>,
    pub r#type: String,
    pub destination: String,
    pub payload: T,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MessageGP<T> {
    pub id: Option<String>,
    pub r#type: String,
    // "text"|"image"|"file"|"audio"|"video"|"contact"|"location"|"button_reply"|"list_reply",
    pub source: String,
    pub payload: T,
    pub sender: Sender,
    pub context: Option<Context>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Sender {
    pub phone: String,
    pub name: String,
    pub country_code: String,
    pub dial_code: String,

}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Context {
    pub id: Option<String>,
    pub gsId: Option<String>,

}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Text {
    pub text: String,
    pub r#type: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Image {
    pub caption: Option<String>,
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Video {
    pub caption: Option<String>,
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct File {
    pub caption: Option<String>,
    pub name: String,
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Audio {
    pub url: String,
    pub contentType: String,
    pub urlExpiry: isize,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ListReply {
    pub title: String,
    pub id: String,
    pub reply: String,
    pub postbackText: String,
    pub description: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ButtonReply {
    pub title: Option<String>,
    pub id: String,
    pub reply: String,
    pub postbackText: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct QuickReply {
    pub text: String,
    pub r#type: String,

}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Location {
    pub longitude: String,
    pub latitude: String,
}













