use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct CreateTemplateResponse {
    pub status: String,
    pub template: Template,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub app_id: String,
    pub element_name: String,
    pub category: String,
    pub template_type: String,
    pub language_code: String,
    pub vertical: String,
    pub content: Option<String>,
    pub footer: Option<String>,
    pub example: Option<String>,
    pub example_header: Option<String>,
    pub allow_template_category_change: Option<bool>,
    pub enable_sample: Option<bool>,
    pub id: Option<String>,
    pub status: Option<String>,
    pub data:String
    // Adicione outros campos conforme necessário
}