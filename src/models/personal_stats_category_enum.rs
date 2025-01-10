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

/// PersonalStatsCategoryEnum : Important! Value 'criminal_offenses' is deprecated, and will be removed on 1st of January 2025.
/// Important! Value 'criminal_offenses' is deprecated, and will be removed on 1st of January 2025.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PersonalStatsCategoryEnum {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "attacking")]
    Attacking,
    #[serde(rename = "battle_stats")]
    BattleStats,
    #[serde(rename = "jobs")]
    Jobs,
    #[serde(rename = "trading")]
    Trading,
    #[serde(rename = "jail")]
    Jail,
    #[serde(rename = "hospital")]
    Hospital,
    #[serde(rename = "finishing_hits")]
    FinishingHits,
    #[serde(rename = "communication")]
    Communication,
    #[serde(rename = "crimes")]
    Crimes,
    #[serde(rename = "bounties")]
    Bounties,
    #[serde(rename = "investments")]
    Investments,
    #[serde(rename = "items")]
    Items,
    #[serde(rename = "travel")]
    Travel,
    #[serde(rename = "drugs")]
    Drugs,
    #[serde(rename = "missions")]
    Missions,
    #[serde(rename = "racing")]
    Racing,
    #[serde(rename = "networth")]
    Networth,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "criminal_offenses")]
    CriminalOffenses,

}

impl std::fmt::Display for PersonalStatsCategoryEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Popular => write!(f, "popular"),
            Self::Attacking => write!(f, "attacking"),
            Self::BattleStats => write!(f, "battle_stats"),
            Self::Jobs => write!(f, "jobs"),
            Self::Trading => write!(f, "trading"),
            Self::Jail => write!(f, "jail"),
            Self::Hospital => write!(f, "hospital"),
            Self::FinishingHits => write!(f, "finishing_hits"),
            Self::Communication => write!(f, "communication"),
            Self::Crimes => write!(f, "crimes"),
            Self::Bounties => write!(f, "bounties"),
            Self::Investments => write!(f, "investments"),
            Self::Items => write!(f, "items"),
            Self::Travel => write!(f, "travel"),
            Self::Drugs => write!(f, "drugs"),
            Self::Missions => write!(f, "missions"),
            Self::Racing => write!(f, "racing"),
            Self::Networth => write!(f, "networth"),
            Self::Other => write!(f, "other"),
            Self::CriminalOffenses => write!(f, "criminal_offenses"),
        }
    }
}

impl Default for PersonalStatsCategoryEnum {
    fn default() -> PersonalStatsCategoryEnum {
        Self::All
    }
}

