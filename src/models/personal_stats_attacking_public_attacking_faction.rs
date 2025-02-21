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
pub struct PersonalStatsAttackingPublicAttackingFaction {
    #[serde(rename = "respect", skip_serializing_if = "Option::is_none")]
    pub respect: Option<i32>,
    #[serde(rename = "retaliations", skip_serializing_if = "Option::is_none")]
    pub retaliations: Option<i32>,
    #[serde(rename = "ranked_war_hits", skip_serializing_if = "Option::is_none")]
    pub ranked_war_hits: Option<i32>,
    #[serde(rename = "raid_hits", skip_serializing_if = "Option::is_none")]
    pub raid_hits: Option<i32>,
    #[serde(rename = "territory", skip_serializing_if = "Option::is_none")]
    pub territory: Option<Box<models::PersonalStatsAttackingPublicAttackingFactionTerritory>>,
}

impl PersonalStatsAttackingPublicAttackingFaction {
    pub fn new() -> PersonalStatsAttackingPublicAttackingFaction {
        PersonalStatsAttackingPublicAttackingFaction {
            respect: None,
            retaliations: None,
            ranked_war_hits: None,
            raid_hits: None,
            territory: None,
        }
    }
}

