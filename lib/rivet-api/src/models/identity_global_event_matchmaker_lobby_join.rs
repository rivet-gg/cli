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
pub struct IdentityGlobalEventMatchmakerLobbyJoin {
    #[serde(rename = "lobby")]
    pub lobby: Box<crate::models::MatchmakerJoinLobby>,
    #[serde(rename = "player")]
    pub player: Box<crate::models::MatchmakerJoinPlayer>,
    #[serde(rename = "ports")]
    pub ports: ::std::collections::HashMap<String, crate::models::MatchmakerJoinPort>,
}

impl IdentityGlobalEventMatchmakerLobbyJoin {
    pub fn new(lobby: crate::models::MatchmakerJoinLobby, player: crate::models::MatchmakerJoinPlayer, ports: ::std::collections::HashMap<String, crate::models::MatchmakerJoinPort>) -> IdentityGlobalEventMatchmakerLobbyJoin {
        IdentityGlobalEventMatchmakerLobbyJoin {
            lobby: Box::new(lobby),
            player: Box::new(player),
            ports,
        }
    }
}


