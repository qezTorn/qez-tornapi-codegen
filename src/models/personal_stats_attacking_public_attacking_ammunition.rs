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
pub struct PersonalStatsAttackingPublicAttackingAmmunition {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "special", skip_serializing_if = "Option::is_none")]
    pub special: Option<i32>,
    #[serde(rename = "hollow_point", skip_serializing_if = "Option::is_none")]
    pub hollow_point: Option<i32>,
    #[serde(rename = "tracer", skip_serializing_if = "Option::is_none")]
    pub tracer: Option<i32>,
    #[serde(rename = "piercing", skip_serializing_if = "Option::is_none")]
    pub piercing: Option<i32>,
    #[serde(rename = "incendiary", skip_serializing_if = "Option::is_none")]
    pub incendiary: Option<i32>,
}

impl PersonalStatsAttackingPublicAttackingAmmunition {
    pub fn new() -> PersonalStatsAttackingPublicAttackingAmmunition {
        PersonalStatsAttackingPublicAttackingAmmunition {
            total: None,
            special: None,
            hollow_point: None,
            tracer: None,
            piercing: None,
            incendiary: None,
        }
    }
}

