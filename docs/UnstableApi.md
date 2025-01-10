# \UnstableApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**faction_chain_get**](UnstableApi.md#faction_chain_get) | **GET** /faction/chain | Get your faction's current chain
[**faction_chain_id_chainreport_get**](UnstableApi.md#faction_chain_id_chainreport_get) | **GET** /faction/{chainId}/chainreport | Get a chain report
[**faction_chainreport_get**](UnstableApi.md#faction_chainreport_get) | **GET** /faction/chainreport | Get your faction's latest chain report
[**faction_chains_get**](UnstableApi.md#faction_chains_get) | **GET** /faction/chains | Get a list of your faction's completed chains
[**faction_crimes_get**](UnstableApi.md#faction_crimes_get) | **GET** /faction/crimes | Get your faction's organized crimes
[**faction_id_chain_get**](UnstableApi.md#faction_id_chain_get) | **GET** /faction/{id}/chain | Get a faction's current chain
[**faction_id_chains_get**](UnstableApi.md#faction_id_chains_get) | **GET** /faction/{id}/chains | Get a list of a faction's completed chains
[**market_id_itemmarket_get**](UnstableApi.md#market_id_itemmarket_get) | **GET** /market/{id}/itemmarket | Get item market listings
[**user_id_personalstats_get**](UnstableApi.md#user_id_personalstats_get) | **GET** /user/{id}/personalstats | Get a player's personal stats
[**user_itemmarket_get**](UnstableApi.md#user_itemmarket_get) | **GET** /user/itemmarket | Get your item market listings for a specific item
[**user_personalstats_get**](UnstableApi.md#user_personalstats_get) | **GET** /user/personalstats | Get your personal stats



## faction_chain_get

> models::FactionOngoingChainResponse faction_chain_get(key)
Get your faction's current chain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::FactionOngoingChainResponse**](FactionOngoingChainResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_chain_id_chainreport_get

> models::FactionChainReportResponse faction_chain_id_chainreport_get(key, chain_id)
Get a chain report

Chain reports for ongoing chains are available only for your own faction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**chain_id** | **i32** | Chain id | [required] |

### Return type

[**models::FactionChainReportResponse**](FactionChainReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_chainreport_get

> models::FactionChainReportResponse faction_chainreport_get(key)
Get your faction's latest chain report

This includes currently ongoing chains.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::FactionChainReportResponse**](FactionChainReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_chains_get

> models::FactionChainsResponse faction_chains_get(key, limit, sort, to, from)
Get a list of your faction's completed chains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest timestamps |  |[default to DESC]
**to** | Option<**i32**> | Timestamp that sets the upper limit for the data returned. Data returned will be up to and including this time |  |
**from** | Option<**i32**> | Timestamp that sets the lower limit for the data returned. Data returned will be after this time |  |

### Return type

[**models::FactionChainsResponse**](FactionChainsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_crimes_get

> models::FactionCrimesResponse faction_crimes_get(key, cat, offset, from, to, sort)
Get your faction's organized crimes

It's possible to get older entries either by timestamp range (from, to) or with offset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |
**cat** | **String** | Category of organized crimes returned. Category 'available' includes both 'recruiting' & 'planning', and category 'completed' includes both 'successful' & 'failure' | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**from** | Option<**i32**> | Returns crimes created after this timestamp |  |
**to** | Option<**i32**> | Returns crimes created before this timestamp |  |
**sort** | Option<**String**> | Direction to sort rows in |  |

### Return type

[**models::FactionCrimesResponse**](FactionCrimesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_id_chain_get

> models::FactionOngoingChainResponse faction_id_chain_get(key, id)
Get a faction's current chain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Faction id | [required] |

### Return type

[**models::FactionOngoingChainResponse**](FactionOngoingChainResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_id_chains_get

> models::FactionChainsResponse faction_id_chains_get(key, id, limit, sort, to, from)
Get a list of a faction's completed chains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Faction id | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest timestamps |  |[default to DESC]
**to** | Option<**i32**> | Timestamp that sets the upper limit for the data returned. Data returned will be up to and including this time |  |
**from** | Option<**i32**> | Timestamp that sets the lower limit for the data returned. Data returned will be after this time |  |

### Return type

[**models::FactionChainsResponse**](FactionChainsResponse.md)

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


## user_id_personalstats_get

> models::UserPersonalStatsResponse user_id_personalstats_get(key, id, cat, stat, timestamp)
Get a player's personal stats

  *  UserPersonalStatsFull is returned only when this selection is requested for the key owner with Limited, Full or Custom key.  *  UserPersonalStatsFullPublic is returned when the requested category is 'all'.  *  UserPersonalStatsPopular is returned when the requested category is 'popular'. Please try to use UserPersonalStatsPopular over UserPersonalStatsFullPublic wherever possible in order to reduce the server load.  *  Otherwise, UserPersonalStatsCategory is returned for the matched category.  *  It's possible to request specific stats via 'stat' parameter. In this case the response will vary depending on the stats requested. Private stats are still available only to the key owner (with Limited or higher key).  *  Additionally, historical stats can also be fetched via 'stat' query parameter, but 'timestamp' parameter must be provided as well. It's only possible to pass up to 10 historical stats at once (the rest is trimmed). When requesting historical stats the response will be of type UserPersonalStatsHistoric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | User id | [required] |
**cat** | Option<[**PersonalStatsCategoryEnum**](.md)> |  |  |
**stat** | Option<[**Vec<models::PersonalStatsStatName>**](models::PersonalStatsStatName.md)> | Stat names (10 maximum). Used to fetch historical stat values |  |
**timestamp** | Option<**i32**> | Returns stats until this timestamp (converted to nearest date). |  |

### Return type

[**models::UserPersonalStatsResponse**](UserPersonalStatsResponse.md)

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


## user_personalstats_get

> models::UserPersonalStatsResponse user_personalstats_get(key, cat, stat, timestamp)
Get your personal stats

  * UserPersonalStatsFull is returned only when this selection is requested with Limited, Full or Custom key access key.  * UserPersonalStatsFullPublic is returned when the requested category is 'all'.  * UserPersonalStatsPopular is returned when the requested category is 'popular'. Please try to use UserPersonalStatsPopular over UserPersonalStatsFullPublic wherever possible in order to reduce the server load.  * Otherwise, UserPersonalStatsCategory is returned for the matched category.  * It's possible to request specific stats via 'stat' parameter. In this case the response will vary depending on the stats requested. Private stats are still available only to the key owner (with Limited or higher key).  * Additionally, historical stats can also be fetched via 'stat' query parameter, but 'timestamp' parameter must be provided as well. It's only possible to pass up to 10 historical stats at once (the rest is trimmed). When requesting historical stats the response will be of type UserPersonalStatsHistoric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**cat** | Option<[**PersonalStatsCategoryEnum**](.md)> | Stats category. Required unless requesting specific stats via 'stat' query parameter |  |
**stat** | Option<[**Vec<models::PersonalStatsStatName>**](models::PersonalStatsStatName.md)> | Stat names (10 maximum). Used to fetch historical stat values |  |
**timestamp** | Option<**i32**> | Returns stats until this timestamp (converted to nearest date). |  |

### Return type

[**models::UserPersonalStatsResponse**](UserPersonalStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

