# \ServersBuildsApi

All URIs are relative to *https://api.rivet.gg*

Method | HTTP request | Description
------------- | ------------- | -------------
[**servers_builds_complete_build**](ServersBuildsApi.md#servers_builds_complete_build) | **POST** /servers/uploads/{upload_id}/complete | 
[**servers_builds_list_builds**](ServersBuildsApi.md#servers_builds_list_builds) | **GET** /servers/builds | 
[**servers_builds_prepare_build**](ServersBuildsApi.md#servers_builds_prepare_build) | **POST** /servers/builds | 



## servers_builds_complete_build

> servers_builds_complete_build(upload_id)


Marks an upload as complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## servers_builds_list_builds

> crate::models::ServersListBuildsResponse servers_builds_list_builds(tags, game_id)


Lists all builds of the game associated with the token used. Can be filtered by tags in the query string.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tags** | Option<**String**> |  |  |
**game_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::ServersListBuildsResponse**](ServersListBuildsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## servers_builds_prepare_build

> crate::models::ServersCreateBuildResponse servers_builds_prepare_build(servers_create_build_request)


Creates a new game build for the given game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**servers_create_build_request** | [**ServersCreateBuildRequest**](ServersCreateBuildRequest.md) |  | [required] |

### Return type

[**crate::models::ServersCreateBuildResponse**](ServersCreateBuildResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

