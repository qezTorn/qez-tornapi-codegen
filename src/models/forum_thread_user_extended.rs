/*
 * Torn API
 *
 *   * The development of Torn's API v2 is still ongoing.  * If selections remain unaltered, they will default to the API v1 version.  * Unlike API v1, API v2 accepts both selections and IDs as path and query parameters.  * If any discrepancies or errors are found, please submit a [bug report](https://www.torn.com/forums.php#/p=forums&f=19&b=0&a=0) on the Torn Forums.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForumThreadUserExtended {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "forum_id", skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i32>,
    #[serde(rename = "posts", skip_serializing_if = "Option::is_none")]
    pub posts: Option<i32>,
    #[serde(rename = "rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    /// Total number of times players have opened this thread.
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<i32>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::ForumThreadAuthor>>,
    #[serde(rename = "last_poster", skip_serializing_if = "Option::is_none")]
    pub last_poster: Option<Box<models::ForumThreadAuthor>>,
    #[serde(rename = "first_post_time", skip_serializing_if = "Option::is_none")]
    pub first_post_time: Option<i32>,
    #[serde(rename = "last_post_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_post_time: Option<Option<i32>>,
    #[serde(rename = "has_poll", skip_serializing_if = "Option::is_none")]
    pub has_poll: Option<bool>,
    #[serde(rename = "is_locked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "is_sticky", skip_serializing_if = "Option::is_none")]
    pub is_sticky: Option<bool>,
    /// Available only when requesting data for yourself (no id or your id) with at least 'Minimal' access type key.
    #[serde(rename = "new_posts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_posts: Option<Option<i32>>,
}

impl ForumThreadUserExtended {
    pub fn new() -> ForumThreadUserExtended {
        ForumThreadUserExtended {
            id: None,
            title: None,
            forum_id: None,
            posts: None,
            rating: None,
            views: None,
            author: None,
            last_poster: None,
            first_post_time: None,
            last_post_time: None,
            has_poll: None,
            is_locked: None,
            is_sticky: None,
            new_posts: None,
        }
    }
}

