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
pub struct CloudGamesValidateGameNamespaceTokenDevelopmentInput {
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// A list of docker ports.
    #[serde(rename = "lobby_ports")]
    pub lobby_ports: Vec<crate::models::CommonsLobbyGroupRuntimeDockerPort>,
}

impl CloudGamesValidateGameNamespaceTokenDevelopmentInput {
    pub fn new(hostname: String, lobby_ports: Vec<crate::models::CommonsLobbyGroupRuntimeDockerPort>) -> CloudGamesValidateGameNamespaceTokenDevelopmentInput {
        CloudGamesValidateGameNamespaceTokenDevelopmentInput {
            hostname,
            lobby_ports,
        }
    }
}

