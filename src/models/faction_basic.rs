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
pub struct FactionBasic {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "tag_image", skip_serializing_if = "Option::is_none")]
    pub tag_image: Option<String>,
    #[serde(rename = "leader_id", skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<i32>,
    #[serde(rename = "co-leader_id", skip_serializing_if = "Option::is_none")]
    pub co_leader_id: Option<i32>,
    #[serde(rename = "respect", skip_serializing_if = "Option::is_none")]
    pub respect: Option<i32>,
    #[serde(rename = "days_old", skip_serializing_if = "Option::is_none")]
    pub days_old: Option<i32>,
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<i32>,
    /// Indicates if the faction is enlisted for ranked wars. Available only with faction AA permissions for your own faction.
    #[serde(rename = "is_enlisted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_enlisted: Option<Option<bool>>,
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<Box<models::FactionRank>>,
    #[serde(rename = "best_chain", skip_serializing_if = "Option::is_none")]
    pub best_chain: Option<i32>,
}

impl FactionBasic {
    pub fn new() -> FactionBasic {
        FactionBasic {
            id: None,
            name: None,
            tag: None,
            tag_image: None,
            leader_id: None,
            co_leader_id: None,
            respect: None,
            days_old: None,
            capacity: None,
            members: None,
            is_enlisted: None,
            rank: None,
            best_chain: None,
        }
    }
}

