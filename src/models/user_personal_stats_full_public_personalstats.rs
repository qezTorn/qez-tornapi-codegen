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
pub struct UserPersonalStatsFullPublicPersonalstats {
    #[serde(rename = "attacking", skip_serializing_if = "Option::is_none")]
    pub attacking: Option<Box<models::PersonalStatsAttackingPublicAttacking>>,
    #[serde(rename = "jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Box<models::PersonalStatsJobsPublicJobs>>,
    #[serde(rename = "trading", skip_serializing_if = "Option::is_none")]
    pub trading: Option<Box<models::PersonalStatsTradingTrading>>,
    #[serde(rename = "jail", skip_serializing_if = "Option::is_none")]
    pub jail: Option<Box<models::PersonalStatsJailJail>>,
    #[serde(rename = "hospital", skip_serializing_if = "Option::is_none")]
    pub hospital: Option<Box<models::PersonalStatsHospitalHospital>>,
    #[serde(rename = "finishing_hits", skip_serializing_if = "Option::is_none")]
    pub finishing_hits: Option<Box<models::PersonalStatsFinishingHitsFinishingHits>>,
    #[serde(rename = "communication", skip_serializing_if = "Option::is_none")]
    pub communication: Option<Box<models::PersonalStatsCommunicationCommunication>>,
    #[serde(rename = "criminal_offenses", skip_serializing_if = "Option::is_none")]
    pub criminal_offenses: Option<Box<models::PersonalStatsCriminalOffensesCriminalOffenses>>,
    #[serde(rename = "crimes", skip_serializing_if = "Option::is_none")]
    pub crimes: Option<Box<models::PersonalStatsCrimesCrimes>>,
    #[serde(rename = "bounties", skip_serializing_if = "Option::is_none")]
    pub bounties: Option<Box<models::PersonalStatsBountiesBounties>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<models::PersonalStatsItemsItems>>,
    #[serde(rename = "travel", skip_serializing_if = "Option::is_none")]
    pub travel: Option<Box<models::PersonalStatsTravelTravel>>,
    #[serde(rename = "drugs", skip_serializing_if = "Option::is_none")]
    pub drugs: Option<Box<models::PersonalStatsDrugsDrugs>>,
    #[serde(rename = "missions", skip_serializing_if = "Option::is_none")]
    pub missions: Option<Box<models::PersonalStatsMissionsMissions>>,
    #[serde(rename = "racing", skip_serializing_if = "Option::is_none")]
    pub racing: Option<Box<models::PersonalStatsRacingRacing>>,
    #[serde(rename = "networth", skip_serializing_if = "Option::is_none")]
    pub networth: Option<Box<models::PersonalStatsNetworthPublicNetworth>>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<Box<models::PersonalStatsOtherOther>>,
}

impl UserPersonalStatsFullPublicPersonalstats {
    pub fn new() -> UserPersonalStatsFullPublicPersonalstats {
        UserPersonalStatsFullPublicPersonalstats {
            attacking: None,
            jobs: None,
            trading: None,
            jail: None,
            hospital: None,
            finishing_hits: None,
            communication: None,
            criminal_offenses: None,
            crimes: None,
            bounties: None,
            items: None,
            travel: None,
            drugs: None,
            missions: None,
            racing: None,
            networth: None,
            other: None,
        }
    }
}

