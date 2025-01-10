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
pub enum ReviveSetting {
    #[serde(rename = "Everyone")]
    Everyone,
    #[serde(rename = "Friends & faction")]
    FriendsAmpersandFaction,
    #[serde(rename = "No one")]
    NoOne,
    #[serde(rename = "Unknown")]
    Unknown,

}

impl std::fmt::Display for ReviveSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Everyone => write!(f, "Everyone"),
            Self::FriendsAmpersandFaction => write!(f, "Friends & faction"),
            Self::NoOne => write!(f, "No one"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for ReviveSetting {
    fn default() -> ReviveSetting {
        Self::Everyone
    }
}

