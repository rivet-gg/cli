# CommonsGameFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**game_id** | **String** | A universally unique identifier. | 
**create_ts** | **String** | RFC3339 timestamp. | 
**name_id** | **String** | A human readable short identifier used to references resources. Different than a `rivet.common#Uuid` because this is intended to be human readable. Different than `rivet.common#DisplayName` because this should not include special characters and be short. | 
**display_name** | **String** | Represent a resource's readable display name. | 
**developer_group_id** | **String** | A universally unique identifier. | 
**total_player_count** | Option<**f64**> | Unsigned 32 bit integer. | [optional]
**logo_url** | Option<**String**> | The URL of this game's logo image. | [optional]
**banner_url** | Option<**String**> | The URL of this game's banner image. | [optional]
**namespaces** | [**Vec<crate::models::CommonsNamespaceSummary>**](CommonsNamespaceSummary.md) | A list of namespace summaries. | 
**versions** | [**Vec<crate::models::CommonsVersionSummary>**](CommonsVersionSummary.md) | A list of version summaries. | 
**available_regions** | [**Vec<crate::models::CommonsRegionSummary>**](CommonsRegionSummary.md) | A list of region summaries. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

