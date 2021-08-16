/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnNamespaceConfig {
    #[serde(rename = "enable_domain_public_auth")]
    pub enable_domain_public_auth: bool,
    #[serde(rename = "domains")]
    pub domains: Vec<crate::models::CdnNamespaceDomain>,
}

impl CdnNamespaceConfig {
    pub fn new(enable_domain_public_auth: bool, domains: Vec<crate::models::CdnNamespaceDomain>) -> CdnNamespaceConfig {
        CdnNamespaceConfig {
            enable_domain_public_auth,
            domains,
        }
    }
}


