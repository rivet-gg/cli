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
pub struct IdentitySetupOutput {
    #[serde(rename = "game_id")]
    pub game_id: uuid::Uuid,
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::IdentityProfile>,
    /// Documentation at https://jwt.io/
    #[serde(rename = "identity_token")]
    pub identity_token: String,
    /// RFC3339 timestamp
    #[serde(rename = "identity_token_expire_ts")]
    pub identity_token_expire_ts: String,
}

impl IdentitySetupOutput {
    pub fn new(game_id: uuid::Uuid, identity: crate::models::IdentityProfile, identity_token: String, identity_token_expire_ts: String) -> IdentitySetupOutput {
        IdentitySetupOutput {
            game_id,
            identity: Box::new(identity),
            identity_token,
            identity_token_expire_ts,
        }
    }
}


