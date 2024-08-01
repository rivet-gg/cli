/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServersServer {
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(rename = "cluster_id")]
    pub cluster_id: uuid::Uuid,
    #[serde(rename = "create_ts")]
    pub create_ts: i64,
    #[serde(rename = "datacenter_id")]
    pub datacenter_id: uuid::Uuid,
    #[serde(rename = "destroy_ts", skip_serializing_if = "Option::is_none")]
    pub destroy_ts: Option<i64>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "game_id")]
    pub game_id: uuid::Uuid,
    #[serde(rename = "image_id")]
    pub image_id: uuid::Uuid,
    /// The duration to wait for in milliseconds before killing the server. This should be set to a safe default, and can be overridden during a DELETE request if needed.
    #[serde(rename = "kill_timeout", skip_serializing_if = "Option::is_none")]
    pub kill_timeout: Option<i64>,
    #[serde(rename = "metadata", deserialize_with = "Option::deserialize")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "network")]
    pub network: Box<crate::models::ServersNetwork>,
    #[serde(rename = "resources")]
    pub resources: Box<crate::models::ServersResources>,
    #[serde(rename = "server_id")]
    pub server_id: uuid::Uuid,
}

impl ServersServer {
    pub fn new(cluster_id: uuid::Uuid, create_ts: i64, datacenter_id: uuid::Uuid, game_id: uuid::Uuid, image_id: uuid::Uuid, metadata: Option<serde_json::Value>, network: crate::models::ServersNetwork, resources: crate::models::ServersResources, server_id: uuid::Uuid) -> ServersServer {
        ServersServer {
            arguments: None,
            cluster_id,
            create_ts,
            datacenter_id,
            destroy_ts: None,
            environment: None,
            game_id,
            image_id,
            kill_timeout: None,
            metadata,
            network: Box::new(network),
            resources: Box::new(resources),
            server_id,
        }
    }
}


