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
pub struct PartyGetPartyFromInviteOutput {
    #[serde(rename = "party")]
    pub party: Box<crate::models::CommonsPartySummary>,
}

impl PartyGetPartyFromInviteOutput {
    pub fn new(party: crate::models::CommonsPartySummary) -> PartyGetPartyFromInviteOutput {
        PartyGetPartyFromInviteOutput {
            party: Box::new(party),
        }
    }
}

