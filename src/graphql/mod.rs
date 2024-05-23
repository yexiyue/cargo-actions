use std::fmt::Display;

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
    pub id: i32,
    pub config: String,
    pub template: String,
    pub is_public: bool,
    pub category_id: i32,
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
    pub category_id: i32,
    pub readme: Option<&'a str>,
    pub source_code_url: Option<&'a str>,
    pub is_public: bool,
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
pub struct UserCreatedTemplates {
    pub templates_by_user: UserTemplates,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "UserTemplates")]
pub struct UserTemplates {
    pub templates: Vec<Template>,
}

impl From<Template> for PathConfig {
    fn from(value: Template) -> Self {
        Self(ActionConfig {
            id: Some(value.id),
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

#[derive(cynic::QueryVariables, Debug)]
pub struct IncreaseTemplateVariables {
    pub id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "IncreaseTemplateVariables")]
pub struct IncreaseTemplate {
    #[arguments(id: $id)]
    pub increase_download_count: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct CategoryAndTags {
    pub categories: CategoryWithPagination,
    pub tags: TagWithPagination,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct TagWithPagination {
    pub tags: Vec<Tag>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CategoryWithPagination {
    pub categories: Vec<Category>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(cynic::QueryVariables, Debug)]
pub struct AddTemplateTagsVariables {
    pub input: TemplateTagInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "AddTemplateTagsVariables")]
pub struct AddTemplateTags {
    #[arguments(input: $input)]
    pub update_tags: String,
}

#[derive(cynic::InputObject, Debug)]
pub struct TemplateTagInput {
    pub tag_id: Vec<i32>,
    pub template_id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct GetUserId {
    pub user: User,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "User")]
pub struct User {
    pub id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct FavoriteTemplates {
    pub favorite_templates: UserTemplates,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct TemplateTagsVariables {
    pub id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "TemplateTagsVariables")]
pub struct TemplateTags {
    #[arguments(id: $id)]
    pub template_tags: Vec<Tag>,
    #[arguments(id: $id)]
    pub template_by_id: Template,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateTemplateVariables<'a> {
    pub id: i32,
    pub input: TemplateUpdateInput<'a>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UpdateTemplateVariables")]
pub struct UpdateTemplate {
    #[arguments(id: $id, input: $input)]
    pub update_template: Template,
}

#[derive(cynic::InputObject, Debug)]
pub struct TemplateUpdateInput<'a> {
    pub name: Option<&'a str>,
    pub config: Option<Json>,
    pub template: Option<&'a str>,
    pub category_id: Option<i32>,
    pub readme: Option<&'a str>,
    pub source_code_url: Option<&'a str>,
    pub is_public: Option<bool>,
}
