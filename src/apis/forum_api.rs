/*
 * Torn API
 *
 *   * The development of Torn's API v2 is still ongoing.  * If selections remain unaltered, they will default to the API v1 version.  * Unlike API v1, API v2 accepts both selections and IDs as path and query parameters.  * If any discrepancies or errors are found, please submit a [bug report](https://www.torn.com/forums.php#/p=forums&f=19&b=0&a=0) on the Torn Forums.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`forum_categories_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumCategoriesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_category_ids_threads_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumCategoryIdsThreadsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_lookup_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumLookupGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_thread_id_posts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumThreadIdPostsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_thread_id_thread_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumThreadIdThreadGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_threads_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumThreadsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`forum_timestamp_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForumTimestampGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_forumfeed_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserForumfeedGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_forumfriends_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserForumfriendsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_forumposts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserForumpostsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_forumsubscribedthreads_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserForumsubscribedthreadsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_forumthreads_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserForumthreadsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_id_forumposts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserIdForumpostsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_id_forumthreads_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserIdForumthreadsGetError {
    UnknownValue(serde_json::Value),
}


pub async fn forum_categories_get(configuration: &configuration::Configuration, key: &str) -> Result<models::ForumCategoriesResponse, Error<ForumCategoriesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/categories", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumCategoriesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn forum_category_ids_threads_get(configuration: &configuration::Configuration, key: &str, category_ids: &str, limit: Option<i32>, sort: Option<&str>, to: Option<i32>, from: Option<i32>) -> Result<models::ForumThreadsResponse, Error<ForumCategoryIdsThreadsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/{categoryIds}/threads", local_var_configuration.base_path, categoryIds=crate::apis::urlencode(category_ids));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumCategoryIdsThreadsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Choose one or more selections (comma separated).
pub async fn forum_get(configuration: &configuration::Configuration, key: &str, selections: Option<Vec<models::ForumSelectionName>>, id: Option<&str>, striptags: Option<&str>, limit: Option<i32>, to: Option<i32>, from: Option<i32>, cat: Option<&str>, sort: Option<&str>, offset: Option<i32>) -> Result<models::ForumGet200Response, Error<ForumGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = selections {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("selections".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("selections", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = striptags {
        local_var_req_builder = local_var_req_builder.query(&[("striptags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cat {
        local_var_req_builder = local_var_req_builder.query(&[("cat", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn forum_lookup_get(configuration: &configuration::Configuration, key: &str) -> Result<models::ForumLookupResponse, Error<ForumLookupGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/lookup", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumLookupGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns 20 posts per page for a specific thread.
pub async fn forum_thread_id_posts_get(configuration: &configuration::Configuration, key: &str, thread_id: i32, offset: Option<i32>, striptags: Option<&str>) -> Result<models::ForumPostsResponse, Error<ForumThreadIdPostsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/{threadId}/posts", local_var_configuration.base_path, threadId=thread_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = striptags {
        local_var_req_builder = local_var_req_builder.query(&[("striptags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumThreadIdPostsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Contains details of a thread including topic content and poll (if any).
pub async fn forum_thread_id_thread_get(configuration: &configuration::Configuration, key: &str, thread_id: i32) -> Result<models::ForumThreadResponse, Error<ForumThreadIdThreadGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/{threadId}/thread", local_var_configuration.base_path, threadId=thread_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumThreadIdThreadGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn forum_threads_get(configuration: &configuration::Configuration, key: &str, limit: Option<i32>, sort: Option<&str>, to: Option<i32>, from: Option<i32>) -> Result<models::ForumThreadsResponse, Error<ForumThreadsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/threads", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumThreadsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn forum_timestamp_get(configuration: &configuration::Configuration, key: &str) -> Result<models::TimestampResponse, Error<ForumTimestampGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/forum/timestamp", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForumTimestampGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This selection returns data visible in 'Feed' section on forum page. Feed is sorted by timestamp descending. Only a maximum of 100 rows are returned.
pub async fn user_forumfeed_get(configuration: &configuration::Configuration, key: &str) -> Result<models::UserForumFeedResponse, Error<UserForumfeedGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/forumfeed", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserForumfeedGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This selection returns data visible in 'Friends' section on forum page. Feed is sorted by timestamp descending. Only a maximum of 100 rows are returned.
pub async fn user_forumfriends_get(configuration: &configuration::Configuration, key: &str) -> Result<models::UserForumFriendsResponse, Error<UserForumfriendsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/forumfriends", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserForumfriendsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns 20 posts per page.
pub async fn user_forumposts_get(configuration: &configuration::Configuration, key: &str, striptags: Option<&str>, limit: Option<i32>, sort: Option<&str>, to: Option<i32>, from: Option<i32>) -> Result<models::UserForumPostsResponse, Error<UserForumpostsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/forumposts", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = striptags {
        local_var_req_builder = local_var_req_builder.query(&[("striptags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserForumpostsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This selection returns data visible in 'Subscribed Threads' section on forum page. Threads are sorted in the same way as on site.
pub async fn user_forumsubscribedthreads_get(configuration: &configuration::Configuration, key: &str) -> Result<models::UserForumSubscribedThreadsResponse, Error<UserForumsubscribedthreadsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/forumsubscribedthreads", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserForumsubscribedthreadsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns 100 threads per page. The field 'new_posts' is also available, indicating the amount of unread posts with a Minimum API key (or higher).
pub async fn user_forumthreads_get(configuration: &configuration::Configuration, key: &str, limit: Option<i32>, sort: Option<&str>, to: Option<i32>, from: Option<i32>) -> Result<models::UserForumThreadsResponse, Error<UserForumthreadsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/forumthreads", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserForumthreadsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns 20 posts per page for a specific player.
pub async fn user_id_forumposts_get(configuration: &configuration::Configuration, key: &str, id: i32, cat: Option<&str>, striptags: Option<&str>, limit: Option<i32>, sort: Option<&str>, to: Option<i32>, from: Option<i32>) -> Result<models::UserForumPostsResponse, Error<UserIdForumpostsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/{id}/forumposts", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = cat {
        local_var_req_builder = local_var_req_builder.query(&[("cat", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = striptags {
        local_var_req_builder = local_var_req_builder.query(&[("striptags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserIdForumpostsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns 100 threads per page for a specific player. When requesting data for the key owner, a field 'new_posts' is also available, indicating the amount of unread posts. Minimum API key is required for that.
pub async fn user_id_forumthreads_get(configuration: &configuration::Configuration, key: &str, id: i32, limit: Option<i32>, sort: Option<&str>, to: Option<i32>, from: Option<i32>) -> Result<models::UserForumThreadsResponse, Error<UserIdForumthreadsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/user/{id}/forumthreads", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserIdForumthreadsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

