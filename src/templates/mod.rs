use std::collections::HashMap;
use reqwest::{Client, Error};
use serde_json::Value;
use crate::models::template::CreateTemplateResponse;

pub async fn get_gupshup_templates(token: &str, app_id: &str) -> Result<Value, Error> {
    let url = format!("https://partner.gupshup.io/partner/account/api/partnerApps{}/templates", app_id);

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
) -> Result<CreateTemplateResponse, Error> {
    let url = format!("https://partner.gupshup.io/partner/app/{}/templates", app_id);
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