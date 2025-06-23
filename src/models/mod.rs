
use serde::{Deserialize, Serialize};


pub mod template;

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



// SEND MODELS

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GupshupMessage {
    pub channel: String,
    pub phone_whatsapp: String,
    #[serde(rename = "src.name")]
    pub src_name: String,
    pub api_key: String,

}

//TEXT

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,

}


// MIDIA IMAGE

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMidia {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_url: String,
    pub preview_url: String,
    pub caption: String,
}

//VIDEO - DOCUMENT - AUDIO

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MidiaType {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub filename: Option<String>,
}


// Button

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonWP<T> {
    #[serde(rename = "type")]
    pub type_field: String,
    pub msgid: String,
    pub content: T, // ContentBT  para botoẽs do tipo texto || ContentMD para midia
    pub options: Vec<OptionButton>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentBT {
    #[serde(rename = "type")]
    pub type_field: String,
    pub header: String,
    pub text: String,
    pub caption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMD {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub caption: String,
}

/*#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionBT {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionButton {
    pub postback_text: String,
    pub title: String,
}


//LIST

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageList {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub body: String,
    pub msgid: Option<String>,
    pub global_buttons: Vec<GlobalButton>,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalButton {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub title: String,
    pub subtitle: String,
    pub options: Vec<OptionList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionList {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub description: Option<String>,
    pub postback_text: Option<String>,
}


// RESPONSE

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMessage {
    pub status: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
}




