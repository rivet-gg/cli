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
pub struct GroupGetGroupProfileOutput {
    #[serde(rename = "group")]
    pub group: Box<crate::models::CommonsGroupProfile>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::CommonsWatchResponse>,
}

impl GroupGetGroupProfileOutput {
    pub fn new(group: crate::models::CommonsGroupProfile, watch: crate::models::CommonsWatchResponse) -> GroupGetGroupProfileOutput {
        GroupGetGroupProfileOutput {
            group: Box::new(group),
            watch: Box::new(watch),
        }
    }
}

