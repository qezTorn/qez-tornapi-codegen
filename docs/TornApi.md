# \TornApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**torn_bounties_get**](TornApi.md#torn_bounties_get) | **GET** /torn/bounties | Get bounties
[**torn_calendar_get**](TornApi.md#torn_calendar_get) | **GET** /torn/calendar | Get calendar information
[**torn_crime_id_subcrimes_get**](TornApi.md#torn_crime_id_subcrimes_get) | **GET** /torn/{crimeId}/subcrimes | Get Subcrimes information
[**torn_crimes_get**](TornApi.md#torn_crimes_get) | **GET** /torn/crimes | Get crimes information
[**torn_factionhof_get**](TornApi.md#torn_factionhof_get) | **GET** /torn/factionhof | Get faction hall of fame positions for a specific category
[**torn_get**](TornApi.md#torn_get) | **GET** /torn | Get any Torn selection
[**torn_hof_get**](TornApi.md#torn_hof_get) | **GET** /torn/hof | Get player hall of fame positions for a specific category
[**torn_log_category_id_logtypes_get**](TornApi.md#torn_log_category_id_logtypes_get) | **GET** /torn/{logCategoryId}/logtypes | Get available log ids for a specific log category
[**torn_logcategories_get**](TornApi.md#torn_logcategories_get) | **GET** /torn/logcategories | Get available log categories
[**torn_logtypes_get**](TornApi.md#torn_logtypes_get) | **GET** /torn/logtypes | Get all available log ids
[**torn_lookup_get**](TornApi.md#torn_lookup_get) | **GET** /torn/lookup | Get all available torn selections
[**torn_timestamp_get**](TornApi.md#torn_timestamp_get) | **GET** /torn/timestamp | Get current server time



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


## torn_calendar_get

> models::TornCalendarResponse torn_calendar_get(key)
Get calendar information

Get the details about competitions & events in the running year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::TornCalendarResponse**](TornCalendarResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_crime_id_subcrimes_get

> models::TornSubcrimesResponse torn_crime_id_subcrimes_get(key, crime_id)
Get Subcrimes information

Return the details about possible actions for a specific crime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**crime_id** | **String** | Crime id | [required] |

### Return type

[**models::TornSubcrimesResponse**](TornSubcrimesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_crimes_get

> models::TornCrimesResponse torn_crimes_get(key)
Get crimes information

Return the details about released crimes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::TornCrimesResponse**](TornCrimesResponse.md)

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


## torn_get

> models::TornGet200Response torn_get(key, selections, id, striptags, limit, to, from, cat, sort, offset)
Get any Torn selection

Choose one or more selections (comma separated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**selections** | Option<[**Vec<models::TornSelectionName>**](models::TornSelectionName.md)> | Selection names |  |
**id** | Option<**String**> | selection id |  |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |
**limit** | Option<**i32**> |  |  |
**to** | Option<**i32**> | Timestamp until when rows are returned |  |
**from** | Option<**i32**> | Timestamp after when rows are returned |  |
**cat** | Option<**String**> | Selection category |  |
**sort** | Option<**String**> | Direction to sort rows in |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::TornGet200Response**](_torn_get_200_response.md)

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


## torn_lookup_get

> models::TornLookupResponse torn_lookup_get(key)
Get all available torn selections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::TornLookupResponse**](TornLookupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## torn_timestamp_get

> models::TimestampResponse torn_timestamp_get(key)
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

