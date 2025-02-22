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
pub struct PersonalStatsCrimesV2Skills {
    #[serde(rename = "search_for_cash", skip_serializing_if = "Option::is_none")]
    pub search_for_cash: Option<i32>,
    #[serde(rename = "bootlegging", skip_serializing_if = "Option::is_none")]
    pub bootlegging: Option<i32>,
    #[serde(rename = "graffiti", skip_serializing_if = "Option::is_none")]
    pub graffiti: Option<i32>,
    #[serde(rename = "shoplifting", skip_serializing_if = "Option::is_none")]
    pub shoplifting: Option<i32>,
    #[serde(rename = "pickpocketing", skip_serializing_if = "Option::is_none")]
    pub pickpocketing: Option<i32>,
    #[serde(rename = "card_skimming", skip_serializing_if = "Option::is_none")]
    pub card_skimming: Option<i32>,
    #[serde(rename = "burglary", skip_serializing_if = "Option::is_none")]
    pub burglary: Option<i32>,
    #[serde(rename = "hustling", skip_serializing_if = "Option::is_none")]
    pub hustling: Option<i32>,
    #[serde(rename = "disposal", skip_serializing_if = "Option::is_none")]
    pub disposal: Option<i32>,
    #[serde(rename = "cracking", skip_serializing_if = "Option::is_none")]
    pub cracking: Option<i32>,
    #[serde(rename = "forgery", skip_serializing_if = "Option::is_none")]
    pub forgery: Option<i32>,
    #[serde(rename = "scamming", skip_serializing_if = "Option::is_none")]
    pub scamming: Option<i32>,
}

impl PersonalStatsCrimesV2Skills {
    pub fn new() -> PersonalStatsCrimesV2Skills {
        PersonalStatsCrimesV2Skills {
            search_for_cash: None,
            bootlegging: None,
            graffiti: None,
            shoplifting: None,
            pickpocketing: None,
            card_skimming: None,
            burglary: None,
            hustling: None,
            disposal: None,
            cracking: None,
            forgery: None,
            scamming: None,
        }
    }
}

