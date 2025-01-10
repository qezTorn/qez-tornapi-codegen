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
pub struct PersonalStatsCriminalOffensesV1 {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "sell_illegal_goods", skip_serializing_if = "Option::is_none")]
    pub sell_illegal_goods: Option<i32>,
    #[serde(rename = "theft", skip_serializing_if = "Option::is_none")]
    pub theft: Option<i32>,
    #[serde(rename = "auto_theft", skip_serializing_if = "Option::is_none")]
    pub auto_theft: Option<i32>,
    #[serde(rename = "drug_deals", skip_serializing_if = "Option::is_none")]
    pub drug_deals: Option<i32>,
    #[serde(rename = "computer", skip_serializing_if = "Option::is_none")]
    pub computer: Option<i32>,
    #[serde(rename = "fraud", skip_serializing_if = "Option::is_none")]
    pub fraud: Option<i32>,
    #[serde(rename = "murder", skip_serializing_if = "Option::is_none")]
    pub murder: Option<i32>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<i32>,
    #[serde(rename = "organized_crimes", skip_serializing_if = "Option::is_none")]
    pub organized_crimes: Option<i32>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl PersonalStatsCriminalOffensesV1 {
    pub fn new() -> PersonalStatsCriminalOffensesV1 {
        PersonalStatsCriminalOffensesV1 {
            total: None,
            sell_illegal_goods: None,
            theft: None,
            auto_theft: None,
            drug_deals: None,
            computer: None,
            fraud: None,
            murder: None,
            other: None,
            organized_crimes: None,
            version: None,
        }
    }
}

