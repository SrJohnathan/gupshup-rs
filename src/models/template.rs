use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTemplateResponse {
    pub status: String,
    pub template: Template,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub appId: String,
    pub elementName: String,
    pub category: String,
    pub templateType: String,
    pub languageCode: String,
    pub vertical: String,
    pub content: Option<String>,
    pub footer: Option<String>,
    pub example: Option<String>,
    pub exampleHeader: Option<String>,
    pub allowTemplateCategoryChange: Option<bool>,
    pub enableSample: Option<bool>,
    pub id: Option<String>,
    pub status: Option<String>,
    pub data:String
    // Adicione outros campos conforme necessário
}