# CommonsGameStatConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**record_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**icon_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**format** | [**crate::models::CommonsGameStatFormatMethod**](CommonsGameStatFormatMethod.md) |  | 
**aggregation** | [**crate::models::CommonsGameStatAggregationMethod**](CommonsGameStatAggregationMethod.md) |  | 
**sorting** | [**crate::models::CommonsGameStatSortingMethod**](CommonsGameStatSortingMethod.md) |  | 
**priority** | **i32** |  | 
**display_name** | **String** |  | 
**postfix_singular** | Option<**String**> | A string appended to the end of a singular game statistic's value. Example: 1 **dollar**. | [optional]
**postfix_plural** | Option<**String**> | A string appended to the end of a game statistic's value that is not exactly 1. Example: 45 **dollars**. | [optional]
**prefix_singular** | Option<**String**> | A string appended to the beginning of a singular game statistic's value. Example: **value** 1. | [optional]
**prefix_plural** | Option<**String**> | A string prepended to the beginning of a game statistic's value that is not exactly 1. Example: **values** 45. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

