# \FactionApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**faction_applications_get**](FactionApi.md#faction_applications_get) | **GET** /faction/applications | Get your faction's applications
[**faction_attacks_get**](FactionApi.md#faction_attacks_get) | **GET** /faction/attacks | Get your faction's detailed attacks
[**faction_attacksfull_get**](FactionApi.md#faction_attacksfull_get) | **GET** /faction/attacksfull | Get your faction's attacks
[**faction_basic_get**](FactionApi.md#faction_basic_get) | **GET** /faction/basic | Get your faction's basic details
[**faction_chain_get**](FactionApi.md#faction_chain_get) | **GET** /faction/chain | Get your faction's current chain
[**faction_chain_id_chainreport_get**](FactionApi.md#faction_chain_id_chainreport_get) | **GET** /faction/{chainId}/chainreport | Get a chain report
[**faction_chainreport_get**](FactionApi.md#faction_chainreport_get) | **GET** /faction/chainreport | Get your faction's latest chain report
[**faction_chains_get**](FactionApi.md#faction_chains_get) | **GET** /faction/chains | Get a list of your faction's completed chains
[**faction_crimes_get**](FactionApi.md#faction_crimes_get) | **GET** /faction/crimes | Get your faction's organized crimes
[**faction_get**](FactionApi.md#faction_get) | **GET** /faction | Get any Faction selection
[**faction_hof_get**](FactionApi.md#faction_hof_get) | **GET** /faction/hof | Get your faction's hall of fame rankings.
[**faction_id_basic_get**](FactionApi.md#faction_id_basic_get) | **GET** /faction/{id}/basic | Get a faction's basic details
[**faction_id_chain_get**](FactionApi.md#faction_id_chain_get) | **GET** /faction/{id}/chain | Get a faction's current chain
[**faction_id_chains_get**](FactionApi.md#faction_id_chains_get) | **GET** /faction/{id}/chains | Get a list of a faction's completed chains
[**faction_id_hof_get**](FactionApi.md#faction_id_hof_get) | **GET** /faction/{id}/hof | Get a faction's hall of fame rankings.
[**faction_id_members_get**](FactionApi.md#faction_id_members_get) | **GET** /faction/{id}/members | Get a list of a faction's members
[**faction_id_wars_get**](FactionApi.md#faction_id_wars_get) | **GET** /faction/{id}/wars | Get a faction's wars & pacts details
[**faction_lookup_get**](FactionApi.md#faction_lookup_get) | **GET** /faction/lookup | Get all available faction selections
[**faction_members_get**](FactionApi.md#faction_members_get) | **GET** /faction/members | Get a list of your faction's members
[**faction_news_get**](FactionApi.md#faction_news_get) | **GET** /faction/news | Get your faction's news details
[**faction_timestamp_get**](FactionApi.md#faction_timestamp_get) | **GET** /faction/timestamp | Get current server time
[**faction_wars_get**](FactionApi.md#faction_wars_get) | **GET** /faction/wars | Get your faction's wars & pacts details



## faction_applications_get

> models::FactionApplicationsResponse faction_applications_get(key)
Get your faction's applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::FactionApplicationsResponse**](FactionApplicationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## faction_basic_get

> models::FactionBasicResponse faction_basic_get(key)
Get your faction's basic details

The 'is_enlisted' value will be populated if you have API faction permissions (with custom, limited or full access keys), otherwise it will be set as null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::FactionBasicResponse**](FactionBasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## faction_get

> models::FactionGet200Response faction_get(key, selections, id, limit, to, from, cat, striptags, sort, offset)
Get any Faction selection

Choose one or more selections (comma separated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**selections** | Option<[**Vec<models::FactionSelectionName>**](models::FactionSelectionName.md)> | Selection names |  |
**id** | Option<**String**> | selection id |  |
**limit** | Option<**i32**> |  |  |
**to** | Option<**i32**> | Timestamp until when rows are returned |  |
**from** | Option<**i32**> | Timestamp after when rows are returned |  |
**cat** | Option<**String**> | Selection category |  |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |
**sort** | Option<**String**> | Direction to sort rows in |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::FactionGet200Response**](_faction_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## faction_id_basic_get

> models::FactionBasicResponse faction_id_basic_get(key, id)
Get a faction's basic details

The 'is_enlisted' value will be populated if you're requesting data for your faction and have faction permissions (with custom, limited or full access keys), otherwise it will be set as null.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Faction id | [required] |

### Return type

[**models::FactionBasicResponse**](FactionBasicResponse.md)

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


## faction_id_members_get

> models::FactionMembersResponse faction_id_members_get(key, id, striptags)
Get a list of a faction's members

The 'revive_setting' value will be populated (not Unknown) if you're requesting data for your own faction and have faction permissions (with custom, limited or full access keys), otherwise it will be set as 'Unknown'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Faction id | [required] |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |[default to true]

### Return type

[**models::FactionMembersResponse**](FactionMembersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_id_wars_get

> models::FactionWarsResponse faction_id_wars_get(key, id)
Get a faction's wars & pacts details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Faction id | [required] |

### Return type

[**models::FactionWarsResponse**](FactionWarsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_lookup_get

> models::FactionLookupResponse faction_lookup_get(key)
Get all available faction selections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::FactionLookupResponse**](FactionLookupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_members_get

> models::FactionMembersResponse faction_members_get(key, striptags)
Get a list of your faction's members

The 'revive_setting' value will be populated (not Unknown) if you have faction permissions (with custom, limited or full access keys), otherwise it will be set as 'Unknown'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |[default to true]

### Return type

[**models::FactionMembersResponse**](FactionMembersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_news_get

> models::FactionNewsResponse faction_news_get(key, cat, striptags, limit, sort, to, from)
Get your faction's news details

It is possible to pass up to 10 categories at the time (comma separated). Categories 'attack', 'depositFunds' and 'giveFunds' are only available with 'Custom', 'Limited' or 'Full' access keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |
**cat** | [**FactionNewsCategory**](.md) | News category type | [required] |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |[default to false]
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest timestamps |  |[default to DESC]
**to** | Option<**i32**> | Timestamp that sets the upper limit for the data returned. Data returned will be up to and including this time |  |
**from** | Option<**i32**> | Timestamp that sets the lower limit for the data returned. Data returned will be after this time |  |

### Return type

[**models::FactionNewsResponse**](FactionNewsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## faction_timestamp_get

> models::TimestampResponse faction_timestamp_get(key)
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


## faction_wars_get

> models::FactionWarsResponse faction_wars_get(key)
Get your faction's wars & pacts details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::FactionWarsResponse**](FactionWarsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

