use std::collections::HashMap;
use reqwest::{Client, Error, Response};
use crate::models::{Audio, ButtonReply, Delivered, Enqueued, Failed, File, Image, ListReply, Location, MessageEvent, MessageGP, ParentMessage, QuickReply, Reaction, Read, ResponseMessage, Sent, Text, Video};
use serde_json::Value;
use crate::extensions::remove_first_nine_from_brazilian_phone;

mod core;
pub mod models;

mod extensions;

pub mod templates; 


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
    Reaction(ParentMessage<MessageGP<Reaction>>),
    
    Unknown,
}

pub fn deserialize(value: &Value) -> MessageType {
    core::init(value)
}




pub use models::GupshupMessage;
use crate::templates::Res;

impl GupshupMessage {
    pub fn new (app:&str,phone_whatsapp:&str,api_key:&str) ->Self {
        Self {
            channel: "whatsapp".to_string(),
            phone_whatsapp: phone_whatsapp.to_string(),
            src_name: app.to_string(),
            api_key: api_key.to_string(),
        }
    }


    pub async fn send_no_time<T: serde::Serialize + Send + 'static>(&self,send_phone:&str,message:T) -> Result<ResponseMessage, String> {
        let req: Client = Client::new();

        let message = serde_json::to_string(&message).unwrap();



        let params =
            [("channel", "whatsapp"),
                ("source", self.phone_whatsapp.as_str()),
                ("destination", send_phone) ,
                ("message", &message),
                ("disablePreview", "true"),
                ("src.name", self.src_name.as_str()  ) ];

        let response = req.post("https://api.gupshup.io/wa/api/v1/msg")
            .header("apikey", self.api_key.as_str())
            .header("Content-Type", "application/x-www-form-urlencoded")
            // .header("Content-Length", content_length.to_string())
            .form(&params)
            .send().await;

        match response {
            Ok(x) => {
              match    x.json::<ResponseMessage>().await {
                  Ok(xx) => {
                      Ok(xx)
                  }
                  Err(ee) => {
                      Err(ee.to_string())
                  }
              }
            }
            Err(e) => { Err(e.to_string()) }
        }
    }
    
    pub async fn set_read_message(&self,app_id:String,message_id:String) -> Result<(), String> {
        let req: Client = Client::new();

        let response = req.put(format!("https://api.gupshup.io/wa/app/{}/msg/{}/read",app_id,message_id).as_str())
            .header("apikey", self.api_key.as_str())
            .send().await;

        match response {
            Ok(x) => {
                if x.status() == reqwest::StatusCode::ACCEPTED {
                    Ok(())
                }else {
                     Err(String::from("Status code unexpected"))
                }
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

}
