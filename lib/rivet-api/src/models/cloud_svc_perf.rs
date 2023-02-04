/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudSvcPerf : A service performance summary.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudSvcPerf {
    /// The name of the service.
    #[serde(rename = "svc_name")]
    pub svc_name: String,
    /// RFC3339 timestamp.
    #[serde(rename = "ts")]
    pub ts: String,
    /// Unsigned 64 bit integer.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// A universally unique identifier.
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// A list of performance spans.
    #[serde(rename = "spans")]
    pub spans: Vec<crate::models::CloudLogsPerfSpan>,
    /// A list of performance marks.
    #[serde(rename = "marks")]
    pub marks: Vec<crate::models::CloudLogsPerfMark>,
}

impl CloudSvcPerf {
    /// A service performance summary.
    pub fn new(svc_name: String, ts: String, spans: Vec<crate::models::CloudLogsPerfSpan>, marks: Vec<crate::models::CloudLogsPerfMark>) -> CloudSvcPerf {
        CloudSvcPerf {
            svc_name,
            ts,
            duration: None,
            req_id: None,
            spans,
            marks,
        }
    }
}

