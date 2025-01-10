# \ItemMarketApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**market_id_itemmarket_get**](ItemMarketApi.md#market_id_itemmarket_get) | **GET** /market/{id}/itemmarket | Get item market listings
[**user_itemmarket_get**](ItemMarketApi.md#user_itemmarket_get) | **GET** /user/itemmarket | Get your item market listings for a specific item



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


## user_itemmarket_get

> models::UserItemMarketResponse user_itemmarket_get(key, offset)
Get your item market listings for a specific item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Limited) | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::UserItemMarketResponse**](UserItemMarketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

