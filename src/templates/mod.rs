use crate::models::template::CreateTemplateResponse;
use reqwest::{Client, Error};
use serde_json::Value;
use std::collections::HashMap;
use serde::Deserialize;

pub async fn get_gupshup_templates(token: &str, app_id: &str) -> Result<Value, Error> {
    let url = format!(
        "https://partner.gupshup.io/partner/app/{}/templates",
        app_id
    );

    let client = Client::new();

    let resp = client
        .get(&url)
        .header("Authorization", token) // token direto, sem "Bearer"
        .send()
        .await?
        .error_for_status()?
        .json::<Value>() // ou ajuste aqui conforme a estrutura real
        .await?;

    Ok(resp)
}
#[derive(Debug, Deserialize)]
pub  struct Res 
{
    status:String,
    pub messageId: String
}


pub async fn send_template_message(
    partner_token: &str,
    app_id: &str,
    source_number: &str,
    destination_number: &str,
    template_id: &str,
    app_name: &str,
    params: Vec<String>,
    message: Option<&str>,         // ✅ agora opcional
    postback_texts: Option<&str>,  // ✅ agora opcional (deve ser um JSON válido!)
) -> Result<String, Error> {
    let client = Client::new();

    let url = format!(
        "https://partner.gupshup.io/partner/app/{}/template/msg",
        app_id
    );

    let params_json = serde_json::to_string(&params).unwrap();
    let template_str = format!(r#"{{"id":"{}","params":{}}}"#, template_id, params_json);

    let mut form = HashMap::new();
    form.insert("channel", "whatsapp");
    form.insert("source", source_number);
    form.insert("sandbox", "false");
    form.insert("destination", destination_number);
    form.insert("template", &template_str);
    form.insert("src.name", app_name);

    if let Some(msg) = message {
        form.insert("message", msg);
    }

    if let Some(postbacks) = postback_texts {
        form.insert("postbackTexts", postbacks); // deve estar em JSON: `[{"index":1,"text":"Oi"}]`
    }

    let res = client
        .post(&url)
        .header("Authorization", partner_token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await?
        .error_for_status()?
        .json::<Res>()
        .await?;

    Ok(res.messageId)
}

pub async fn create_gupshup_text_template(
    token: &str,
    app_id: &str,
    element_name: &str,
    language_code: Option<&str>,
    content: &str,
    category: &str,
    vertical: &str,
    template_type: &str, // ex: "TEXT"
    example: &str,
    footer: Option<&str>,
    example_header: Option<&str>,
    allow_category_change: Option<bool>,
    enable_sample: Option<bool>,
    buttons: Option<&str>, // NOVO
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
        params.insert("exampleHeader", exh);
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

    let resp = client
        .post(&url)
        .header("Authorization", token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .error_for_status()?
        .json::<CreateTemplateResponse>()
        .await?;

    Ok(resp)
}
