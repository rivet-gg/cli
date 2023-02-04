/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommonsLobbyGroupRuntimeDocker : A game mode runtime running through Docker.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommonsLobbyGroupRuntimeDocker {
    #[serde(rename = "build_id", skip_serializing_if = "Option::is_none")]
    pub build_id: Option<uuid::Uuid>,
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "env_vars")]
    pub env_vars: Vec<crate::models::CommonsLobbyGroupRuntimeDockerEnvVar>,
    #[serde(rename = "network_mode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<crate::models::CommonsNetworkMode>,
    #[serde(rename = "ports")]
    pub ports: Vec<crate::models::CommonsLobbyGroupRuntimeDockerPort>,
}

impl CommonsLobbyGroupRuntimeDocker {
    /// A game mode runtime running through Docker.
    pub fn new(args: Vec<String>, env_vars: Vec<crate::models::CommonsLobbyGroupRuntimeDockerEnvVar>, ports: Vec<crate::models::CommonsLobbyGroupRuntimeDockerPort>) -> CommonsLobbyGroupRuntimeDocker {
        CommonsLobbyGroupRuntimeDocker {
            build_id: None,
            args,
            env_vars,
            network_mode: None,
            ports,
        }
    }
}

