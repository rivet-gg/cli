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
pub struct EeBackendCreateResponse {
    #[serde(rename = "backend")]
    pub backend: Box<crate::models::EeBackendBackend>,
}

impl EeBackendCreateResponse {
    pub fn new(backend: crate::models::EeBackendBackend) -> EeBackendCreateResponse {
        EeBackendCreateResponse {
            backend: Box::new(backend),
        }
    }
}


