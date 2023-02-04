# CommonsGameProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | 
**name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**display_name** | **String** | Represent a resource's readable display name. | 
**logo_url** | Option<**String**> | The URL of this game's logo image. | [optional]
**banner_url** | Option<**String**> | The URL of this game's banner image. | [optional]
**url** | **String** | The URL to this game's website. | 
**developer** | [**crate::models::CommonsGroupSummary**](CommonsGroupSummary.md) |  | 
**tags** | **Vec<String>** | A list of game tags. | 
**description** | **String** | A description of the given game. | 
**platforms** | [**Vec<crate::models::CommonsGamePlatformLink>**](CommonsGamePlatformLink.md) | A list of platform links. | 
**recommended_groups** | [**Vec<crate::models::CommonsGroupSummary>**](CommonsGroupSummary.md) | A list of group summaries. | 
**identity_leaderboard_categories** | [**Vec<crate::models::CommonsGameLeaderboardCategory>**](CommonsGameLeaderboardCategory.md) | A list of game leaderboard categories. | 
**group_leaderboard_categories** | [**Vec<crate::models::CommonsGameLeaderboardCategory>**](CommonsGameLeaderboardCategory.md) | A list of game leaderboard categories. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

