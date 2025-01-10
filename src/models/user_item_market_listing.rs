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
pub struct UserItemMarketListing {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(rename = "average_price", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<i32>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "is_anonymous", skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Amount remaining in the inventory.
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<i32>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<models::UserItemMarkeListingItemDetails>>,
}

impl UserItemMarketListing {
    pub fn new() -> UserItemMarketListing {
        UserItemMarketListing {
            id: None,
            price: None,
            average_price: None,
            amount: None,
            is_anonymous: None,
            available: None,
            item: None,
        }
    }
}

