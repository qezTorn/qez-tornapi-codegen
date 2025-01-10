# \BountiesApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**torn_bounties_get**](BountiesApi.md#torn_bounties_get) | **GET** /torn/bounties | Get bounties
[**user_bounties_get**](BountiesApi.md#user_bounties_get) | **GET** /user/bounties | Get bounties placed on you
[**user_id_bounties_get**](BountiesApi.md#user_id_bounties_get) | **GET** /user/{id}/bounties | Get bounties placed on a specific user



## torn_bounties_get

> models::TornBountiesResponse torn_bounties_get(key, limit, offset)
Get bounties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::TornBountiesResponse**](TornBountiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_bounties_get

> models::UserBountiesResponse user_bounties_get(key)
Get bounties placed on you

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::UserBountiesResponse**](UserBountiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_bounties_get

> models::UserBountiesResponse user_id_bounties_get(key, id)
Get bounties placed on a specific user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | User id | [required] |

### Return type

[**models::UserBountiesResponse**](UserBountiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

