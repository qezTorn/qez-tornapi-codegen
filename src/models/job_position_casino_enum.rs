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
pub enum JobPositionCasinoEnum {
    #[serde(rename = "Gaming Consultant")]
    GamingConsultant,
    #[serde(rename = "Marketing Manager")]
    MarketingManager,
    #[serde(rename = "Revenue Manager")]
    RevenueManager,
    #[serde(rename = "Casino Manager")]
    CasinoManager,
    #[serde(rename = "Casino President")]
    CasinoPresident,

}

impl std::fmt::Display for JobPositionCasinoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GamingConsultant => write!(f, "Gaming Consultant"),
            Self::MarketingManager => write!(f, "Marketing Manager"),
            Self::RevenueManager => write!(f, "Revenue Manager"),
            Self::CasinoManager => write!(f, "Casino Manager"),
            Self::CasinoPresident => write!(f, "Casino President"),
        }
    }
}

impl Default for JobPositionCasinoEnum {
    fn default() -> JobPositionCasinoEnum {
        Self::GamingConsultant
    }
}

