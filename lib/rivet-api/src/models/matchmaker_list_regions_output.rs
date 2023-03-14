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
#[serde(deny_unknown_fields)]
pub struct MatchmakerListRegionsOutput {
    #[serde(rename = "regions")]
    pub regions: Vec<crate::models::MatchmakerRegionInfo>,
}

impl MatchmakerListRegionsOutput {
    pub fn new(regions: Vec<crate::models::MatchmakerRegionInfo>) -> MatchmakerListRegionsOutput {
        MatchmakerListRegionsOutput {
            regions,
        }
    }
}


