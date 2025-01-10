# \RacingApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**racing_cars_get**](RacingApi.md#racing_cars_get) | **GET** /racing/cars | Get cars and their racing stats
[**racing_carupgrades_get**](RacingApi.md#racing_carupgrades_get) | **GET** /racing/carupgrades | Get all possible car upgrades
[**racing_get**](RacingApi.md#racing_get) | **GET** /racing | Get any Racing selection
[**racing_lookup_get**](RacingApi.md#racing_lookup_get) | **GET** /racing/lookup | Get all available racing selections
[**racing_race_id_race_get**](RacingApi.md#racing_race_id_race_get) | **GET** /racing/{raceId}/race | Get specific race details
[**racing_races_get**](RacingApi.md#racing_races_get) | **GET** /racing/races | Get races
[**racing_timestamp_get**](RacingApi.md#racing_timestamp_get) | **GET** /racing/timestamp | Get current server time
[**racing_track_id_records_get**](RacingApi.md#racing_track_id_records_get) | **GET** /racing/{trackId}/records | Get track records
[**racing_tracks_get**](RacingApi.md#racing_tracks_get) | **GET** /racing/tracks | Get race tracks and descriptions
[**user_enlistedcars_get**](RacingApi.md#user_enlistedcars_get) | **GET** /user/enlistedcars | Get user enlisted cars
[**user_races_get**](RacingApi.md#user_races_get) | **GET** /user/races | Get user races



## racing_cars_get

> models::RacingCarsResponse racing_cars_get(key)
Get cars and their racing stats

Returns the stat details about racing cars.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::RacingCarsResponse**](RacingCarsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_carupgrades_get

> models::RacingCarUpgradesResponse racing_carupgrades_get(key)
Get all possible car upgrades

Returns the details about all possible car upgrades.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::RacingCarUpgradesResponse**](RacingCarUpgradesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_get

> models::RacingGet200Response racing_get(key, selections, id, limit, to, from, cat, sort, offset)
Get any Racing selection

Choose one or more selections (comma separated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**selections** | Option<[**Vec<models::RacingSelectionName>**](models::RacingSelectionName.md)> | Selection names |  |
**id** | Option<**String**> | selection id |  |
**limit** | Option<**i32**> |  |  |
**to** | Option<**i32**> | Timestamp until when rows are returned |  |
**from** | Option<**i32**> | Timestamp after when rows are returned |  |
**cat** | Option<**String**> | Selection category |  |
**sort** | Option<**String**> | Direction to sort rows in |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::RacingGet200Response**](_racing_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_lookup_get

> models::RacingLookupResponse racing_lookup_get(key)
Get all available racing selections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::RacingLookupResponse**](RacingLookupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_race_id_race_get

> models::RacingRaceDetailsResponse racing_race_id_race_get(key, id)
Get specific race details

Returns the details of a race.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | Race id | [required] |

### Return type

[**models::RacingRaceDetailsResponse**](RacingRaceDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_races_get

> models::RacingRacesResponse racing_races_get(key, limit, sort, to, from, cat)
Get races

Returns a list of races, ordered by race start timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by schedule.start field |  |[default to DESC]
**to** | Option<**i32**> | Timestamp until when started races are returned (schedule.start) |  |
**from** | Option<**i32**> | Timestamp after when started races are returned (scheduled.start) |  |
**cat** | Option<**String**> | Category of races returned |  |[default to custom]

### Return type

[**models::RacingRacesResponse**](RacingRacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_timestamp_get

> models::TimestampResponse racing_timestamp_get(key)
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


## racing_track_id_records_get

> models::RacingTrackRecordsResponse racing_track_id_records_get(key, track_id, cat)
Get track records

Returns a list of 10 best lap records for the chosen track and car class. Results are cached globally 1 hour.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**track_id** | **i32** | Track id | [required] |
**cat** | [**RaceClassEnum**](.md) | Car class | [required] |

### Return type

[**models::RacingTrackRecordsResponse**](RacingTrackRecordsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## racing_tracks_get

> models::RacingTracksResponse racing_tracks_get(key)
Get race tracks and descriptions

Returns the details about racing tracks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::RacingTracksResponse**](RacingTracksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_enlistedcars_get

> models::UserEnlistedCarsResponse user_enlistedcars_get(key)
Get user enlisted cars

Returns a list of all user enlisted cars.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::UserEnlistedCarsResponse**](UserEnlistedCarsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_races_get

> models::UserRacesResponse user_races_get(key, limit, sort, to, from, cat)
Get user races

Returns a list of user races, ordered by race start timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by schedule.start field |  |[default to DESC]
**to** | Option<**i32**> | Timestamp until when started races are returned (schedule.start) |  |
**from** | Option<**i32**> | Timestamp after when started races are returned (scheduled.start) |  |
**cat** | Option<**String**> | Category of races returned |  |[default to custom]

### Return type

[**models::UserRacesResponse**](UserRacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

