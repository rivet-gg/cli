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
pub struct EeCloudBackendProjectsEnvsGetEventsResponse {
    /// Sorted old to new.
    #[serde(rename = "events")]
    pub events: Vec<serde_json::Value>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl EeCloudBackendProjectsEnvsGetEventsResponse {
    pub fn new(events: Vec<serde_json::Value>, watch: crate::models::WatchResponse) -> EeCloudBackendProjectsEnvsGetEventsResponse {
        EeCloudBackendProjectsEnvsGetEventsResponse {
            events,
            watch: Box::new(watch),
        }
    }
}


