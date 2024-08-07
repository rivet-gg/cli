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
pub struct AdminClustersCreateDatacenterRequest {
    #[serde(rename = "build_delivery_method")]
    pub build_delivery_method: crate::models::AdminClustersBuildDeliveryMethod,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "name_id")]
    pub name_id: String,
    #[serde(rename = "prebakes_enabled")]
    pub prebakes_enabled: bool,
    #[serde(rename = "provider")]
    pub provider: crate::models::AdminClustersProvider,
    #[serde(rename = "provider_datacenter_id")]
    pub provider_datacenter_id: String,
}

impl AdminClustersCreateDatacenterRequest {
    pub fn new(build_delivery_method: crate::models::AdminClustersBuildDeliveryMethod, display_name: String, name_id: String, prebakes_enabled: bool, provider: crate::models::AdminClustersProvider, provider_datacenter_id: String) -> AdminClustersCreateDatacenterRequest {
        AdminClustersCreateDatacenterRequest {
            build_delivery_method,
            display_name,
            name_id,
            prebakes_enabled,
            provider,
            provider_datacenter_id,
        }
    }
}


