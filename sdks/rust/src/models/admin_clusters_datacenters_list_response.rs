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
pub struct AdminClustersDatacentersListResponse {
    #[serde(rename = "datacenters")]
    pub datacenters: Vec<crate::models::AdminDatacenter>,
}

impl AdminClustersDatacentersListResponse {
    pub fn new(datacenters: Vec<crate::models::AdminDatacenter>) -> AdminClustersDatacentersListResponse {
        AdminClustersDatacentersListResponse {
            datacenters,
        }
    }
}


