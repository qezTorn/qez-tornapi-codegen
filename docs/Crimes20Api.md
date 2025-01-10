# \Crimes20Api

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**torn_crime_id_subcrimes_get**](Crimes20Api.md#torn_crime_id_subcrimes_get) | **GET** /torn/{crimeId}/subcrimes | Get Subcrimes information
[**torn_crimes_get**](Crimes20Api.md#torn_crimes_get) | **GET** /torn/crimes | Get crimes information
[**user_crime_id_crimes_get**](Crimes20Api.md#user_crime_id_crimes_get) | **GET** /user/{crimeId}/crimes | Get your crime statistics



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


## user_crime_id_crimes_get

> models::UserCrimesResponse user_crime_id_crimes_get(key, crime_id)
Get your crime statistics

Return the details and statistics about for a specific crime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |
**crime_id** | **String** | Crime id | [required] |

### Return type

[**models::UserCrimesResponse**](UserCrimesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

