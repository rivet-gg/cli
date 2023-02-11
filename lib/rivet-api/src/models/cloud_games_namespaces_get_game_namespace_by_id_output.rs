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
pub struct CloudGamesNamespacesGetGameNamespaceByIdOutput {
    #[serde(rename = "namespace")]
    pub namespace: Box<crate::models::CloudNamespaceFull>,
}

impl CloudGamesNamespacesGetGameNamespaceByIdOutput {
    pub fn new(namespace: crate::models::CloudNamespaceFull) -> CloudGamesNamespacesGetGameNamespaceByIdOutput {
        CloudGamesNamespacesGetGameNamespaceByIdOutput {
            namespace: Box::new(namespace),
        }
    }
}

