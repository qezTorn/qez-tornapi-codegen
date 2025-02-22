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
pub struct UserCrime {
    #[serde(rename = "nerve_spent", skip_serializing_if = "Option::is_none")]
    pub nerve_spent: Option<i32>,
    #[serde(rename = "skill", skip_serializing_if = "Option::is_none")]
    pub skill: Option<i32>,
    #[serde(rename = "progression_bonus", skip_serializing_if = "Option::is_none")]
    pub progression_bonus: Option<i32>,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Box<models::UserCrimeRewards>>,
    #[serde(rename = "attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Box<models::UserCrimeAttempts>>,
    #[serde(rename = "uniques", skip_serializing_if = "Option::is_none")]
    pub uniques: Option<Vec<models::UserCrimeUniques>>,
    #[serde(rename = "miscellaneous", skip_serializing_if = "Option::is_none")]
    pub miscellaneous: Option<Box<models::UserCrimeMiscellaneous>>,
}

impl UserCrime {
    pub fn new() -> UserCrime {
        UserCrime {
            nerve_spent: None,
            skill: None,
            progression_bonus: None,
            rewards: None,
            attempts: None,
            uniques: None,
            miscellaneous: None,
        }
    }
}

