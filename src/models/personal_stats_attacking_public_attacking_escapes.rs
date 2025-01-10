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
pub struct PersonalStatsAttackingPublicAttackingEscapes {
    #[serde(rename = "player", skip_serializing_if = "Option::is_none")]
    pub player: Option<i32>,
    #[serde(rename = "foes", skip_serializing_if = "Option::is_none")]
    pub foes: Option<i32>,
}

impl PersonalStatsAttackingPublicAttackingEscapes {
    pub fn new() -> PersonalStatsAttackingPublicAttackingEscapes {
        PersonalStatsAttackingPublicAttackingEscapes {
            player: None,
            foes: None,
        }
    }
}

