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
pub struct TornFactionHofResponse {
    #[serde(rename = "factionhof", skip_serializing_if = "Option::is_none")]
    pub factionhof: Option<Vec<models::TornFactionHof>>,
    #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
    pub _metadata: Option<Box<models::RequestMetadataWithLinks>>,
}

impl TornFactionHofResponse {
    pub fn new() -> TornFactionHofResponse {
        TornFactionHofResponse {
            factionhof: None,
            _metadata: None,
        }
    }
}

