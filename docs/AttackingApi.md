# \AttackingApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**faction_attacks_get**](AttackingApi.md#faction_attacks_get) | **GET** /faction/attacks | Get your faction's detailed attacks
[**faction_attacksfull_get**](AttackingApi.md#faction_attacksfull_get) | **GET** /faction/attacksfull | Get your faction's attacks



## faction_attacks_get

> models::FactionAttacksResponse faction_attacks_get(key, limit, sort, to, from)
Get your faction's detailed attacks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Limited) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest timestamps |  |[default to DESC]
**to** | Option<**i32**> | Timestamp that sets the upper limit for the data returned. Data returned will be up to and including this time |  |
**from** | Option<**i32**> | Timestamp that sets the lower limit for the data returned. Data returned will be after this time |  |

### Return type

[**models::FactionAttacksResponse**](FactionAttacksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_attacksfull_get

> models::FactionAttacksFullResponse faction_attacksfull_get(key, limit, sort, to, from)
Get your faction's attacks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Limited) | [required] |
**limit** | Option<**i32**> |  |  |[default to 1000]
**sort** | Option<**String**> | Sorted by the greatest timestamps |  |[default to DESC]
**to** | Option<**i32**> | Timestamp that sets the upper limit for the data returned. Data returned will be up to and including this time |  |
**from** | Option<**i32**> | Timestamp that sets the lower limit for the data returned. Data returned will be after this time |  |

### Return type

[**models::FactionAttacksFullResponse**](FactionAttacksFullResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

