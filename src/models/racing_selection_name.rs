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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RacingSelectionName {
    #[serde(rename = "cars")]
    Cars,
    #[serde(rename = "carupgrades")]
    Carupgrades,
    #[serde(rename = "lookup")]
    Lookup,
    #[serde(rename = "race")]
    Race,
    #[serde(rename = "races")]
    Races,
    #[serde(rename = "records")]
    Records,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "tracks")]
    Tracks,

}

impl std::fmt::Display for RacingSelectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cars => write!(f, "cars"),
            Self::Carupgrades => write!(f, "carupgrades"),
            Self::Lookup => write!(f, "lookup"),
            Self::Race => write!(f, "race"),
            Self::Races => write!(f, "races"),
            Self::Records => write!(f, "records"),
            Self::Timestamp => write!(f, "timestamp"),
            Self::Tracks => write!(f, "tracks"),
        }
    }
}

impl Default for RacingSelectionName {
    fn default() -> RacingSelectionName {
        Self::Cars
    }
}

