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
pub struct PersonalStatsInvestmentsInvestmentsStocks {
    #[serde(rename = "profits", skip_serializing_if = "Option::is_none")]
    pub profits: Option<i32>,
    #[serde(rename = "losses", skip_serializing_if = "Option::is_none")]
    pub losses: Option<i32>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<i32>,
    #[serde(rename = "net_profits", skip_serializing_if = "Option::is_none")]
    pub net_profits: Option<i32>,
    #[serde(rename = "payouts", skip_serializing_if = "Option::is_none")]
    pub payouts: Option<i32>,
}

impl PersonalStatsInvestmentsInvestmentsStocks {
    pub fn new() -> PersonalStatsInvestmentsInvestmentsStocks {
        PersonalStatsInvestmentsInvestmentsStocks {
            profits: None,
            losses: None,
            fees: None,
            net_profits: None,
            payouts: None,
        }
    }
}

