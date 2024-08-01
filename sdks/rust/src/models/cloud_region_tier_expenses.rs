/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudRegionTierExpenses : Region tier expenses.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudRegionTierExpenses {
    /// Amount of expenses for this region tier (in hundred-thousandths USD, 100,000 = $1.00).
    #[serde(rename = "expenses")]
    pub expenses: f64,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "lobby_group_name_id")]
    pub lobby_group_name_id: String,
    #[serde(rename = "namespace_id")]
    pub namespace_id: uuid::Uuid,
    #[serde(rename = "region_id")]
    pub region_id: uuid::Uuid,
    /// A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short.
    #[serde(rename = "tier_name_id")]
    pub tier_name_id: String,
    /// How long a region tier has been active (in milliseconds).
    #[serde(rename = "uptime")]
    pub uptime: f64,
}

impl CloudRegionTierExpenses {
    /// Region tier expenses.
    pub fn new(expenses: f64, lobby_group_name_id: String, namespace_id: uuid::Uuid, region_id: uuid::Uuid, tier_name_id: String, uptime: f64) -> CloudRegionTierExpenses {
        CloudRegionTierExpenses {
            expenses,
            lobby_group_name_id,
            namespace_id,
            region_id,
            tier_name_id,
            uptime,
        }
    }
}


