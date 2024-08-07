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
pub struct AdminClustersListClustersResponse {
    #[serde(rename = "clusters")]
    pub clusters: Vec<crate::models::AdminClustersCluster>,
}

impl AdminClustersListClustersResponse {
    pub fn new(clusters: Vec<crate::models::AdminClustersCluster>) -> AdminClustersListClustersResponse {
        AdminClustersListClustersResponse {
            clusters,
        }
    }
}


