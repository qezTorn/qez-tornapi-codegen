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
pub struct FactionCrimeSlot {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "item_requirement", skip_serializing_if = "Option::is_none")]
    pub item_requirement: Option<Box<models::FactionCrimeSlotItemRequirement>>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::FactionCrimeSlotUser>>,
    #[serde(rename = "success_chance", skip_serializing_if = "Option::is_none")]
    pub success_chance: Option<i32>,
}

impl FactionCrimeSlot {
    pub fn new() -> FactionCrimeSlot {
        FactionCrimeSlot {
            position: None,
            item_requirement: None,
            user_id: None,
            user: None,
            success_chance: None,
        }
    }
}

