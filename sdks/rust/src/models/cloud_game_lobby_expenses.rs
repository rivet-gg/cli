/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudGameLobbyExpenses : Game lobby expenses.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGameLobbyExpenses {
    /// A list of multiple region tier expenses.
    #[serde(rename = "expenses")]
    pub expenses: Vec<crate::models::CloudRegionTierExpenses>,
    #[serde(rename = "game")]
    pub game: Box<crate::models::GameHandle>,
    /// A list of namespace summaries.
    #[serde(rename = "namespaces")]
    pub namespaces: Vec<crate::models::CloudNamespaceSummary>,
}

impl CloudGameLobbyExpenses {
    /// Game lobby expenses.
    pub fn new(expenses: Vec<crate::models::CloudRegionTierExpenses>, game: crate::models::GameHandle, namespaces: Vec<crate::models::CloudNamespaceSummary>) -> CloudGameLobbyExpenses {
        CloudGameLobbyExpenses {
            expenses,
            game: Box::new(game),
            namespaces,
        }
    }
}


