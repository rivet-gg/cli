# MatchmakerRegionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region_id** | **String** | A human readable short identifier used to references resources. Different than a `uuid` because this is intended to be human readable. Different than `DisplayName` because this should not include special characters and be short. | 
**provider_display_name** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**region_display_name** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**datacenter_coord** | [**crate::models::CommonsCoord**](CommonsCoord.md) |  | 
**datacenter_distance_from_client** | [**crate::models::CommonsDistance**](CommonsDistance.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

