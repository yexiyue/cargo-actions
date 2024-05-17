use actions_templates::ActionConfig;

use crate::path_configs::{PathConfig, PathConfigs};

#[cynic::schema("main")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct QueryTemplateVariables {
    pub id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "QueryTemplateVariables")]
pub struct QueryTemplate {
    #[arguments(id: $id)]
    pub template_by_id: Option<Template>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Template {
    pub config: String,
    pub template: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UploadTemplateVariables<'a> {
    pub input: TemplateCreateInput<'a>,
}

#[derive(cynic::InputObject, Debug)]
pub struct TemplateCreateInput<'a> {
    pub name: &'a str,
    pub config: Json,
    pub template: &'a str,
    pub user_id: i32,
    pub category_id: Option<i32>,
    pub readme: Option<&'a str>,
    pub source_code_url: Option<&'a str>,
}

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "JSON")]
pub struct Json(pub String);

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UploadTemplateVariables")]
pub struct UploadTemplate {
    #[arguments(input: $input)]
    pub create_template: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct UserTemplates {
    pub templates_by_user: Vec<Template>,
}

impl From<Template> for PathConfig {
    fn from(value: Template) -> Self {
        Self(ActionConfig {
            config: value.config.into(),
            readme: None,
            template: value.template.into(),
        })
    }
}

impl From<Vec<Template>> for PathConfigs {
    fn from(value: Vec<Template>) -> Self {
        Self {
            inner: value.into_iter().map(PathConfig::from).collect(),
        }
    }
}
