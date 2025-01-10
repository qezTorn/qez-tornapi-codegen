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
pub struct ItemMarketListingItemStats {
    #[serde(rename = "damage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub damage: Option<Option<f32>>,
    #[serde(rename = "accuracy", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<Option<f32>>,
    #[serde(rename = "armor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub armor: Option<Option<f32>>,
}

impl ItemMarketListingItemStats {
    pub fn new() -> ItemMarketListingItemStats {
        ItemMarketListingItemStats {
            damage: None,
            accuracy: None,
            armor: None,
        }
    }
}

