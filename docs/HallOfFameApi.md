# \HallOfFameApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**faction_hof_get**](HallOfFameApi.md#faction_hof_get) | **GET** /faction/hof | Get your faction's hall of fame rankings.
[**faction_id_hof_get**](HallOfFameApi.md#faction_id_hof_get) | **GET** /faction/{id}/hof | Get a faction's hall of fame rankings.
[**torn_factionhof_get**](HallOfFameApi.md#torn_factionhof_get) | **GET** /torn/factionhof | Get faction hall of fame positions for a specific category
[**torn_hof_get**](HallOfFameApi.md#torn_hof_get) | **GET** /torn/hof | Get player hall of fame positions for a specific category
[**user_hof_get**](HallOfFameApi.md#user_hof_get) | **GET** /user/hof | Get your hall of fame rankings
[**user_id_hof_get**](HallOfFameApi.md#user_id_hof_get) | **GET** /user/{id}/hof | Get hall of fame rankings for a specific player



## faction_hof_get

> models::FactionHofResponse faction_hof_get(key)
Get your faction's hall of fame rankings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::FactionHofResponse**](FactionHofResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_id_hof_get

> models::FactionHofResponse faction_id_hof_get(key, id)
Get a faction's hall of fame rankings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Faction id | [required] |

### Return type

[**models::FactionHofResponse**](FactionHofResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_factionhof_get

> models::TornFactionHofResponse torn_factionhof_get(key, cat, limit, offset)
Get faction hall of fame positions for a specific category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**cat** | [**TornFactionHofCategory**](.md) | Leaderboards category | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::TornFactionHofResponse**](TornFactionHofResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_hof_get

> models::TornHofResponse torn_hof_get(key, cat, limit, offset)
Get player hall of fame positions for a specific category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**cat** | [**TornHofCategory**](.md) | Leaderboards category | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::TornHofResponse**](TornHofResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_hof_get

> models::UserHofResponse user_hof_get(key)
Get your hall of fame rankings

When requesting selection with Limited, Full or Custom key, battle_stats selection will be populated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::UserHofResponse**](UserHofResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_hof_get

> models::UserHofResponse user_id_hof_get(key, id)
Get hall of fame rankings for a specific player

The battle_stats selection will be populated only when requesting selection with Limited, Full or Custom key and for yourself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | User id | [required] |

### Return type

[**models::UserHofResponse**](UserHofResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

