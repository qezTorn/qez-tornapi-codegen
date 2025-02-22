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
pub enum TornHofCategory {
    #[serde(rename = "level")]
    Level,
    #[serde(rename = "busts")]
    Busts,
    #[serde(rename = "rank")]
    Rank,
    #[serde(rename = "traveltime")]
    Traveltime,
    #[serde(rename = "workstats")]
    Workstats,
    #[serde(rename = "networth")]
    Networth,
    #[serde(rename = "revives")]
    Revives,
    #[serde(rename = "defends")]
    Defends,
    #[serde(rename = "offences")]
    Offences,
    #[serde(rename = "attacks")]
    Attacks,
    #[serde(rename = "awards")]
    Awards,
    #[serde(rename = "racingwins")]
    Racingwins,
    #[serde(rename = "racingpoints")]
    Racingpoints,
    #[serde(rename = "racingskill")]
    Racingskill,

}

impl std::fmt::Display for TornHofCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Level => write!(f, "level"),
            Self::Busts => write!(f, "busts"),
            Self::Rank => write!(f, "rank"),
            Self::Traveltime => write!(f, "traveltime"),
            Self::Workstats => write!(f, "workstats"),
            Self::Networth => write!(f, "networth"),
            Self::Revives => write!(f, "revives"),
            Self::Defends => write!(f, "defends"),
            Self::Offences => write!(f, "offences"),
            Self::Attacks => write!(f, "attacks"),
            Self::Awards => write!(f, "awards"),
            Self::Racingwins => write!(f, "racingwins"),
            Self::Racingpoints => write!(f, "racingpoints"),
            Self::Racingskill => write!(f, "racingskill"),
        }
    }
}

impl Default for TornHofCategory {
    fn default() -> TornHofCategory {
        Self::Level
    }
}

