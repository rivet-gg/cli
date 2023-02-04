/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsGroupBillingSummary : A group billing summary.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsGroupBillingSummary {
    /// A list of multiple game lobby expenses.
    #[serde(rename = "games")]
    pub games: Vec<crate::models::CommonsGameLobbyExpenses>,
    /// A group's available balance.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
}

impl CommonsGroupBillingSummary {
    /// A group billing summary.
    pub fn new(games: Vec<crate::models::CommonsGameLobbyExpenses>) -> CommonsGroupBillingSummary {
        CommonsGroupBillingSummary {
            games,
            balance: None,
        }
    }
}

