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
pub struct FactionOngoingChain {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Seconds until chain breaks.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "modifier", skip_serializing_if = "Option::is_none")]
    pub modifier: Option<f32>,
    /// Timestamp until when chain is on cooldown.
    #[serde(rename = "cooldown", skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i32>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
}

impl FactionOngoingChain {
    pub fn new() -> FactionOngoingChain {
        FactionOngoingChain {
            id: None,
            current: None,
            max: None,
            timeout: None,
            modifier: None,
            cooldown: None,
            start: None,
            end: None,
        }
    }
}

