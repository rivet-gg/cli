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
pub struct CloudGamesNamespacesCreateGameNamespaceOutput {
    /// A universally unique identifier.
    #[serde(rename = "namespace_id")]
    pub namespace_id: String,
}

impl CloudGamesNamespacesCreateGameNamespaceOutput {
    pub fn new(namespace_id: String) -> CloudGamesNamespacesCreateGameNamespaceOutput {
        CloudGamesNamespacesCreateGameNamespaceOutput {
            namespace_id,
        }
    }
}


