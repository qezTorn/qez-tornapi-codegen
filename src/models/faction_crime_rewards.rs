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

/// FactionCrimeRewards : Details about the crime rewards. Available only for crimes with 'Successful' status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactionCrimeRewards {
    #[serde(rename = "money", skip_serializing_if = "Option::is_none")]
    pub money: Option<i32>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::FactionCrimeRewardItem>>,
    #[serde(rename = "respect", skip_serializing_if = "Option::is_none")]
    pub respect: Option<i32>,
}

impl FactionCrimeRewards {
    /// Details about the crime rewards. Available only for crimes with 'Successful' status.
    pub fn new() -> FactionCrimeRewards {
        FactionCrimeRewards {
            money: None,
            items: None,
            respect: None,
        }
    }
}

