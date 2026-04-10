use crate::models::template::CreateTemplateResponse;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io;

pub async fn get_gupshup_templates(token: &str, app_id: &str) -> Result<Value, Error> {
    let url = format!(
        "https://partner.gupshup.io/partner/app/{}/templates",
        app_id
    );

    let client = Client::new();

    let resp = client
        .get(&url)
        .header("Authorization", token)
        .send()
        .await?
        .error_for_status()?
        .json::<Value>()
        .await?;

    Ok(resp)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Res {
    status: String,
    #[serde(alias = "message_id")]
    pub message_id: String,
}

pub async fn send_template_message(
    app_access_token: &str,
    app_id: &str,
    source_number: &str,
    destination_number: &str,
    template_id: &str,
    app_name: &str,
    params: Vec<String>,
    message: Option<&str>,
    postback_texts: Option<&str>,
) -> Result<String, Error> {
    let client = Client::new();

    let url = format!(
        "https://partner.gupshup.io/partner/app/{}/template/msg",
        app_id
    );

    let params_json = serde_json::to_string(&params).unwrap();
    let template_str = format!(r#"{{"id":"{}","params":{}}}"#, template_id, params_json);
    let message_payload = message
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|msg| {
            if msg.starts_with('{') {
                msg.to_string()
            } else {
                serde_json::json!({
                    "type": "text",
                    "text": msg,
                })
                .to_string()
            }
        })
        .unwrap_or_else(|| {
            serde_json::json!({
                "type": "text",
                "text": "Mensagem de template",
            })
            .to_string()
        });

    let mut form = HashMap::new();
    form.insert("channel", "whatsapp");
    form.insert("source", source_number);
    form.insert("sandbox", "false");
    form.insert("destination", destination_number);
    form.insert("template", &template_str);
    form.insert("message", &message_payload);
    form.insert("src.name", app_name);

    if let Some(postbacks) = postback_texts {
        form.insert("postbackTexts", postbacks);
    }

    let res = client
        .post(&url)
        .header("token", app_access_token)
        .header("accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await?
        .error_for_status()?
        .json::<Res>()
        .await?;

    Ok(res.message_id)
}

pub async fn create_gupshup_text_template(
    token: &str,
    app_id: &str,
    element_name: &str,
    language_code: Option<&str>,
    content: &str,
    category: &str,
    vertical: &str,
    template_type: &str,
    example: &str,
    footer: Option<&str>,
    example_header: Option<&str>,
    allow_category_change: Option<bool>,
    enable_sample: Option<bool>,
    buttons: Option<&str>,
) -> Result<CreateTemplateResponse, Error> {
    let url = format!(
        "https://partner.gupshup.io/partner/app/{}/templates",
        app_id
    );
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("elementName", element_name);
    if let Some(lang) = language_code {
        params.insert("languageCode", lang);
    }
    params.insert("content", content);
    params.insert("category", category);
    params.insert("vertical", vertical);
    params.insert("templateType", template_type);
    params.insert("example", example);
    if let Some(f) = footer {
        params.insert("footer", f);
    }
    if let Some(exh) = example_header {
        params.insert("header", exh);
    }
    if let Some(true) = allow_category_change {
        params.insert("allowTemplateCategoryChange", "true");
    }
    if let Some(true) = enable_sample {
        params.insert("enableSample", "true");
    }

    if let Some(btns) = buttons {
        params.insert("buttons", btns);
    }

    let response = client
        .post(&url)
        .header("Authorization", token)
        .header("accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(io::Error::other(format!(
            "Gupshup create template failed: status={} body={}",
            status,
            body
        ))
        .into());
    }

    let resp = serde_json::from_str::<CreateTemplateResponse>(&body)
        .map_err(io::Error::other)?;

    Ok(resp)
}
