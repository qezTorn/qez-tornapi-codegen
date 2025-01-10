# \LogsApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**torn_log_category_id_logtypes_get**](LogsApi.md#torn_log_category_id_logtypes_get) | **GET** /torn/{logCategoryId}/logtypes | Get available log ids for a specific log category
[**torn_logcategories_get**](LogsApi.md#torn_logcategories_get) | **GET** /torn/logcategories | Get available log categories
[**torn_logtypes_get**](LogsApi.md#torn_logtypes_get) | **GET** /torn/logtypes | Get all available log ids



## torn_log_category_id_logtypes_get

> models::TornLogTypesResponse torn_log_category_id_logtypes_get(key, log_category_id)
Get available log ids for a specific log category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**log_category_id** | **i32** | Log category id | [required] |

### Return type

[**models::TornLogTypesResponse**](TornLogTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_logcategories_get

> models::TornLogCategoriesResponse torn_logcategories_get(key)
Get available log categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::TornLogCategoriesResponse**](TornLogCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_logtypes_get

> models::TornLogTypesResponse torn_logtypes_get(key)
Get all available log ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::TornLogTypesResponse**](TornLogTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

