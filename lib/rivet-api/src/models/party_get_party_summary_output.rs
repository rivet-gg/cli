/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PartyGetPartySummaryOutput {
    #[serde(rename = "party")]
    pub party: Box<crate::models::CommonsPartySummary>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::CommonsWatchResponse>,
}

impl PartyGetPartySummaryOutput {
    pub fn new(party: crate::models::CommonsPartySummary, watch: crate::models::CommonsWatchResponse) -> PartyGetPartySummaryOutput {
        PartyGetPartySummaryOutput {
            party: Box::new(party),
            watch: Box::new(watch),
        }
    }
}

