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
pub struct RaceCar {
    #[serde(rename = "car_item_id", skip_serializing_if = "Option::is_none")]
    pub car_item_id: Option<i32>,
    #[serde(rename = "car_item_name", skip_serializing_if = "Option::is_none")]
    pub car_item_name: Option<String>,
    #[serde(rename = "top_speed", skip_serializing_if = "Option::is_none")]
    pub top_speed: Option<i32>,
    #[serde(rename = "acceleration", skip_serializing_if = "Option::is_none")]
    pub acceleration: Option<i32>,
    #[serde(rename = "braking", skip_serializing_if = "Option::is_none")]
    pub braking: Option<i32>,
    #[serde(rename = "dirt", skip_serializing_if = "Option::is_none")]
    pub dirt: Option<i32>,
    #[serde(rename = "handling", skip_serializing_if = "Option::is_none")]
    pub handling: Option<i32>,
    #[serde(rename = "safety", skip_serializing_if = "Option::is_none")]
    pub safety: Option<i32>,
    #[serde(rename = "tarmac", skip_serializing_if = "Option::is_none")]
    pub tarmac: Option<i32>,
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<models::RaceClassEnum>,
}

impl RaceCar {
    pub fn new() -> RaceCar {
        RaceCar {
            car_item_id: None,
            car_item_name: None,
            top_speed: None,
            acceleration: None,
            braking: None,
            dirt: None,
            handling: None,
            safety: None,
            tarmac: None,
            class: None,
        }
    }
}

