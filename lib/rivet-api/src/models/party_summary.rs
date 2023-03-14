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
#[serde(deny_unknown_fields)]
pub struct PartySummary {
    #[serde(rename = "activity")]
    pub activity: Box<crate::models::PartyActivity>,
    /// RFC3339 timestamp
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    #[serde(rename = "external")]
    pub external: Box<crate::models::PartyExternalLinks>,
    #[serde(rename = "members")]
    pub members: Vec<crate::models::PartyMemberSummary>,
    #[serde(rename = "party_id")]
    pub party_id: uuid::Uuid,
    #[serde(rename = "party_size")]
    pub party_size: i32,
    #[serde(rename = "publicity")]
    pub publicity: Box<crate::models::PartyPublicity>,
    #[serde(rename = "thread_id")]
    pub thread_id: uuid::Uuid,
}

impl PartySummary {
    pub fn new(activity: crate::models::PartyActivity, create_ts: String, external: crate::models::PartyExternalLinks, members: Vec<crate::models::PartyMemberSummary>, party_id: uuid::Uuid, party_size: i32, publicity: crate::models::PartyPublicity, thread_id: uuid::Uuid) -> PartySummary {
        PartySummary {
            activity: Box::new(activity),
            create_ts,
            external: Box::new(external),
            members,
            party_id,
            party_size,
            publicity: Box::new(publicity),
            thread_id,
        }
    }
}


