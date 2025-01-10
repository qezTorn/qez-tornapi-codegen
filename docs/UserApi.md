# \UserApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_bounties_get**](UserApi.md#user_bounties_get) | **GET** /user/bounties | Get bounties placed on you
[**user_calendar_get**](UserApi.md#user_calendar_get) | **GET** /user/calendar | Get your competition's event start time
[**user_crime_id_crimes_get**](UserApi.md#user_crime_id_crimes_get) | **GET** /user/{crimeId}/crimes | Get your crime statistics
[**user_enlistedcars_get**](UserApi.md#user_enlistedcars_get) | **GET** /user/enlistedcars | Get user enlisted cars
[**user_forumfeed_get**](UserApi.md#user_forumfeed_get) | **GET** /user/forumfeed | Get updates on your threads and posts
[**user_forumfriends_get**](UserApi.md#user_forumfriends_get) | **GET** /user/forumfriends | Get updates on your friends' activity
[**user_forumposts_get**](UserApi.md#user_forumposts_get) | **GET** /user/forumposts | Get your posts
[**user_forumsubscribedthreads_get**](UserApi.md#user_forumsubscribedthreads_get) | **GET** /user/forumsubscribedthreads | Get updates on threads you subscribed to
[**user_forumthreads_get**](UserApi.md#user_forumthreads_get) | **GET** /user/forumthreads | Get your threads
[**user_get**](UserApi.md#user_get) | **GET** /user | Get any User selection
[**user_hof_get**](UserApi.md#user_hof_get) | **GET** /user/hof | Get your hall of fame rankings
[**user_id_bounties_get**](UserApi.md#user_id_bounties_get) | **GET** /user/{id}/bounties | Get bounties placed on a specific user
[**user_id_forumposts_get**](UserApi.md#user_id_forumposts_get) | **GET** /user/{id}/forumposts | Get posts for a specific player
[**user_id_forumthreads_get**](UserApi.md#user_id_forumthreads_get) | **GET** /user/{id}/forumthreads | Get threads for a specific player
[**user_id_hof_get**](UserApi.md#user_id_hof_get) | **GET** /user/{id}/hof | Get hall of fame rankings for a specific player
[**user_id_personalstats_get**](UserApi.md#user_id_personalstats_get) | **GET** /user/{id}/personalstats | Get a player's personal stats
[**user_itemmarket_get**](UserApi.md#user_itemmarket_get) | **GET** /user/itemmarket | Get your item market listings for a specific item
[**user_jobranks_get**](UserApi.md#user_jobranks_get) | **GET** /user/jobranks | Get your starter job positions
[**user_lookup_get**](UserApi.md#user_lookup_get) | **GET** /user/lookup | Get all available user selections
[**user_personalstats_get**](UserApi.md#user_personalstats_get) | **GET** /user/personalstats | Get your personal stats
[**user_races_get**](UserApi.md#user_races_get) | **GET** /user/races | Get user races
[**user_timestamp_get**](UserApi.md#user_timestamp_get) | **GET** /user/timestamp | Get current server time



## user_bounties_get

> models::UserBountiesResponse user_bounties_get(key)
Get bounties placed on you

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::UserBountiesResponse**](UserBountiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_calendar_get

> models::UserCalendarResponse user_calendar_get(key)
Get your competition's event start time

Only available to yourself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::UserCalendarResponse**](UserCalendarResponse.md)

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


## user_forumfeed_get

> models::UserForumFeedResponse user_forumfeed_get(key)
Get updates on your threads and posts

This selection returns data visible in 'Feed' section on forum page. Feed is sorted by timestamp descending. Only a maximum of 100 rows are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::UserForumFeedResponse**](UserForumFeedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_forumfriends_get

> models::UserForumFriendsResponse user_forumfriends_get(key)
Get updates on your friends' activity

This selection returns data visible in 'Friends' section on forum page. Feed is sorted by timestamp descending. Only a maximum of 100 rows are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::UserForumFriendsResponse**](UserForumFriendsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_forumposts_get

> models::UserForumPostsResponse user_forumposts_get(key, striptags, limit, sort, to, from)
Get your posts

Returns 20 posts per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |[default to true]
**limit** | Option<**i32**> |  |  |[default to 20]
**sort** | Option<**String**> | Sorted by post created timestamp |  |[default to DESC]
**to** | Option<**i32**> | Returns posts created before this timestamp |  |
**from** | Option<**i32**> | Returns posts created after this timestamp |  |

### Return type

[**models::UserForumPostsResponse**](UserForumPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_forumsubscribedthreads_get

> models::UserForumSubscribedThreadsResponse user_forumsubscribedthreads_get(key)
Get updates on threads you subscribed to

This selection returns data visible in 'Subscribed Threads' section on forum page. Threads are sorted in the same way as on site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::UserForumSubscribedThreadsResponse**](UserForumSubscribedThreadsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_forumthreads_get

> models::UserForumThreadsResponse user_forumthreads_get(key, limit, sort, to, from)
Get your threads

Returns 100 threads per page. The field 'new_posts' is also available, indicating the amount of unread posts with a Minimum API key (or higher).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest of first_post_time and last_post_time timestamps |  |[default to DESC]
**to** | Option<**i32**> | Returns threads created before this timestamp |  |
**from** | Option<**i32**> | Returns threads created after this timestamp |  |

### Return type

[**models::UserForumThreadsResponse**](UserForumThreadsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get

> models::UserGet200Response user_get(key, selections, id, limit, to, from, cat, stat, striptags, sort, offset)
Get any User selection

Choose one or more selections (comma separated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**selections** | Option<[**Vec<models::UserSelectionName>**](models::UserSelectionName.md)> | Selection names |  |
**id** | Option<**String**> | selection id |  |
**limit** | Option<**i32**> |  |  |
**to** | Option<**i32**> | Timestamp until when rows are returned |  |
**from** | Option<**i32**> | Timestamp after when rows are returned |  |
**cat** | Option<**String**> | Selection category |  |
**stat** | Option<**String**> | Selection stat |  |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |
**sort** | Option<**String**> | Direction to sort rows in |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::UserGet200Response**](_user_get_200_response.md)

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


## user_id_bounties_get

> models::UserBountiesResponse user_id_bounties_get(key, id)
Get bounties placed on a specific user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | User id | [required] |

### Return type

[**models::UserBountiesResponse**](UserBountiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_forumposts_get

> models::UserForumPostsResponse user_id_forumposts_get(key, id, cat, striptags, limit, sort, to, from)
Get posts for a specific player

Returns 20 posts per page for a specific player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | User id | [required] |
**cat** | Option<**String**> | This parameter is being replaced with 'stripTags' parameter and will be removed on 1st December 2024. Determines if the 'content' field returns raw HTML or plain text |  |[default to plain]
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |[default to true]
**limit** | Option<**i32**> |  |  |[default to 20]
**sort** | Option<**String**> | Sorted by post created timestamp |  |[default to DESC]
**to** | Option<**i32**> | Returns posts created before this timestamp |  |
**from** | Option<**i32**> | Returns posts created after this timestamp |  |

### Return type

[**models::UserForumPostsResponse**](UserForumPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_forumthreads_get

> models::UserForumThreadsResponse user_id_forumthreads_get(key, id, limit, sort, to, from)
Get threads for a specific player

Returns 100 threads per page for a specific player. When requesting data for the key owner, a field 'new_posts' is also available, indicating the amount of unread posts. Minimum API key is required for that.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**id** | **i32** | User id | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest of first_post_time and last_post_time timestamps |  |[default to DESC]
**to** | Option<**i32**> | Returns threads created before this timestamp |  |
**from** | Option<**i32**> | Returns threads created after this timestamp |  |

### Return type

[**models::UserForumThreadsResponse**](UserForumThreadsResponse.md)

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


## user_jobranks_get

> models::UserJobRanksResponse user_jobranks_get(key)
Get your starter job positions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Minimal) | [required] |

### Return type

[**models::UserJobRanksResponse**](UserJobRanksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_lookup_get

> models::UserLookupResponse user_lookup_get(key)
Get all available user selections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::UserLookupResponse**](UserLookupResponse.md)

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


## user_timestamp_get

> models::TimestampResponse user_timestamp_get(key)
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

