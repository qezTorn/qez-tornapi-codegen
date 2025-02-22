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
pub struct Bounty {
    #[serde(rename = "target_id", skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i32>,
    #[serde(rename = "target_name", skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    #[serde(rename = "target_level", skip_serializing_if = "Option::is_none")]
    pub target_level: Option<i32>,
    /// If the bounty is anonymous this field is null.
    #[serde(rename = "lister_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lister_id: Option<Option<Box<i32>>>,
    /// If the bounty is anonymous this field is null.
    #[serde(rename = "lister_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lister_name: Option<Option<String>>,
    #[serde(rename = "reward", skip_serializing_if = "Option::is_none")]
    pub reward: Option<i32>,
    #[serde(rename = "reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "is_anonymous", skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(rename = "valid_until", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<i32>,
}

impl Bounty {
    pub fn new() -> Bounty {
        Bounty {
            target_id: None,
            target_name: None,
            target_level: None,
            lister_id: None,
            lister_name: None,
            reward: None,
            reason: None,
            quantity: None,
            is_anonymous: None,
            valid_until: None,
        }
    }
}

