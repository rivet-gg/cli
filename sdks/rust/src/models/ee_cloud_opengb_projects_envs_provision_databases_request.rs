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
pub struct EeCloudOpengbProjectsEnvsProvisionDatabasesRequest {
    #[serde(rename = "modules")]
    pub modules: ::std::collections::HashMap<String, crate::models::EeOpengbModule>,
}

impl EeCloudOpengbProjectsEnvsProvisionDatabasesRequest {
    pub fn new(modules: ::std::collections::HashMap<String, crate::models::EeOpengbModule>) -> EeCloudOpengbProjectsEnvsProvisionDatabasesRequest {
        EeCloudOpengbProjectsEnvsProvisionDatabasesRequest {
            modules,
        }
    }
}


