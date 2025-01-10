# \MarketApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**market_get**](MarketApi.md#market_get) | **GET** /market | Get any Market selection
[**market_id_itemmarket_get**](MarketApi.md#market_id_itemmarket_get) | **GET** /market/{id}/itemmarket | Get item market listings
[**market_lookup_get**](MarketApi.md#market_lookup_get) | **GET** /market/lookup | Get all available market selections
[**market_timestamp_get**](MarketApi.md#market_timestamp_get) | **GET** /market/timestamp | Get current server time



## market_get

> models::MarketGet200Response market_get(key, selections, id, bonus, cat, sort, offset)
Get any Market selection

Choose one or more selections (comma separated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**selections** | Option<[**Vec<models::MarketSelectionName>**](models::MarketSelectionName.md)> | Selection names |  |
**id** | Option<**String**> | selection id |  |
**bonus** | Option<[**WeaponBonusEnum**](.md)> | Used to filter weapons with a specific bonus |  |
**cat** | Option<**String**> | Selection category |  |
**sort** | Option<**String**> | Direction to sort rows in |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::MarketGet200Response**](_market_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## market_id_itemmarket_get

> models::MarketItemMarketResponse market_id_itemmarket_get(key, id, bonus, offset)
Get item market listings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Item id | [required] |
**bonus** | Option<[**WeaponBonusEnum**](.md)> | Used to filter weapons with a specific bonus. |  |
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::MarketItemMarketResponse**](MarketItemMarketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## market_lookup_get

> models::MarketLookupResponse market_lookup_get(key)
Get all available market selections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::MarketLookupResponse**](MarketLookupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## market_timestamp_get

> models::TimestampResponse market_timestamp_get(key)
Get current server time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::TimestampResponse**](TimestampResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

