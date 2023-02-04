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
pub struct PartyGetPartyProfileOutput {
    #[serde(rename = "party")]
    pub party: Box<crate::models::CommonsPartyProfile>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::CommonsWatchResponse>,
}

impl PartyGetPartyProfileOutput {
    pub fn new(party: crate::models::CommonsPartyProfile, watch: crate::models::CommonsWatchResponse) -> PartyGetPartyProfileOutput {
        PartyGetPartyProfileOutput {
            party: Box::new(party),
            watch: Box::new(watch),
        }
    }
}

