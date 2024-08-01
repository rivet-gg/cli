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
pub struct ServersPortRouting {
    #[serde(rename = "game_guard", skip_serializing_if = "Option::is_none")]
    pub game_guard: Option<serde_json::Value>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<serde_json::Value>,
}

impl ServersPortRouting {
    pub fn new() -> ServersPortRouting {
        ServersPortRouting {
            game_guard: None,
            host: None,
        }
    }
}


