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
pub struct TornGet200Response {
    #[serde(rename = "subcrimes", skip_serializing_if = "Option::is_none")]
    pub subcrimes: Option<Vec<models::TornSubcrime>>,
    #[serde(rename = "crimes", skip_serializing_if = "Option::is_none")]
    pub crimes: Option<Vec<models::TornCrime>>,
    #[serde(rename = "calendar", skip_serializing_if = "Option::is_none")]
    pub calendar: Option<Box<models::TornCalendarResponseCalendar>>,
    #[serde(rename = "hof", skip_serializing_if = "Option::is_none")]
    pub hof: Option<Vec<models::TornHof>>,
    #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
    pub _metadata: Option<Box<models::RequestMetadataWithLinks>>,
    #[serde(rename = "factionhof", skip_serializing_if = "Option::is_none")]
    pub factionhof: Option<Vec<models::TornFactionHof>>,
    #[serde(rename = "logtypes", skip_serializing_if = "Option::is_none")]
    pub logtypes: Option<Vec<models::TornLog>>,
    #[serde(rename = "logcategories", skip_serializing_if = "Option::is_none")]
    pub logcategories: Option<Vec<models::TornLogCategory>>,
    #[serde(rename = "bounties", skip_serializing_if = "Option::is_none")]
    pub bounties: Option<Vec<models::Bounty>>,
    #[serde(rename = "selections", skip_serializing_if = "Option::is_none")]
    pub selections: Option<Vec<models::TornSelectionName>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
}

impl TornGet200Response {
    pub fn new() -> TornGet200Response {
        TornGet200Response {
            subcrimes: None,
            crimes: None,
            calendar: None,
            hof: None,
            _metadata: None,
            factionhof: None,
            logtypes: None,
            logcategories: None,
            bounties: None,
            selections: None,
            timestamp: None,
        }
    }
}

