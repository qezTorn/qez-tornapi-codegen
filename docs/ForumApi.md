# \ForumApi

All URIs are relative to *https://api.torn.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forum_categories_get**](ForumApi.md#forum_categories_get) | **GET** /forum/categories | Get publicly available forum categories
[**forum_category_ids_threads_get**](ForumApi.md#forum_category_ids_threads_get) | **GET** /forum/{categoryIds}/threads | Get threads for specific public forum category or categories
[**forum_get**](ForumApi.md#forum_get) | **GET** /forum | Get any Forum selection
[**forum_lookup_get**](ForumApi.md#forum_lookup_get) | **GET** /forum/lookup | Get all available forum selections
[**forum_thread_id_posts_get**](ForumApi.md#forum_thread_id_posts_get) | **GET** /forum/{threadId}/posts | Get specific forum thread posts
[**forum_thread_id_thread_get**](ForumApi.md#forum_thread_id_thread_get) | **GET** /forum/{threadId}/thread | Get specific thread details
[**forum_threads_get**](ForumApi.md#forum_threads_get) | **GET** /forum/threads | Get threads across all forum categories
[**forum_timestamp_get**](ForumApi.md#forum_timestamp_get) | **GET** /forum/timestamp | Get current server time
[**user_forumfeed_get**](ForumApi.md#user_forumfeed_get) | **GET** /user/forumfeed | Get updates on your threads and posts
[**user_forumfriends_get**](ForumApi.md#user_forumfriends_get) | **GET** /user/forumfriends | Get updates on your friends' activity
[**user_forumposts_get**](ForumApi.md#user_forumposts_get) | **GET** /user/forumposts | Get your posts
[**user_forumsubscribedthreads_get**](ForumApi.md#user_forumsubscribedthreads_get) | **GET** /user/forumsubscribedthreads | Get updates on threads you subscribed to
[**user_forumthreads_get**](ForumApi.md#user_forumthreads_get) | **GET** /user/forumthreads | Get your threads
[**user_id_forumposts_get**](ForumApi.md#user_id_forumposts_get) | **GET** /user/{id}/forumposts | Get posts for a specific player
[**user_id_forumthreads_get**](ForumApi.md#user_id_forumthreads_get) | **GET** /user/{id}/forumthreads | Get threads for a specific player



## forum_categories_get

> models::ForumCategoriesResponse forum_categories_get(key)
Get publicly available forum categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::ForumCategoriesResponse**](ForumCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_category_ids_threads_get

> models::ForumThreadsResponse forum_category_ids_threads_get(key, category_ids, limit, sort, to, from)
Get threads for specific public forum category or categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**category_ids** | **String** | Category id or a list of category ids (comma separated) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest of first_post_time and last_post_time timestamps |  |[default to DESC]
**to** | Option<**i32**> | Returns threads created before this timestamp |  |
**from** | Option<**i32**> | Returns threads created after this timestamp |  |

### Return type

[**models::ForumThreadsResponse**](ForumThreadsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_get

> models::ForumGet200Response forum_get(key, selections, id, striptags, limit, to, from, cat, sort, offset)
Get any Forum selection

Choose one or more selections (comma separated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**selections** | Option<[**Vec<models::ForumSelectionName>**](models::ForumSelectionName.md)> | Selection names |  |
**id** | Option<**String**> | selection id |  |
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |
**limit** | Option<**i32**> |  |  |
**to** | Option<**i32**> | Timestamp until when rows are returned |  |
**from** | Option<**i32**> | Timestamp after when rows are returned |  |
**cat** | Option<**String**> | Selection category |  |
**sort** | Option<**String**> | Direction to sort rows in |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::ForumGet200Response**](_forum_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_lookup_get

> models::ForumLookupResponse forum_lookup_get(key)
Get all available forum selections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |

### Return type

[**models::ForumLookupResponse**](ForumLookupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_thread_id_posts_get

> models::ForumPostsResponse forum_thread_id_posts_get(key, thread_id, offset, striptags)
Get specific forum thread posts

Returns 20 posts per page for a specific thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**thread_id** | **i32** | Thread id | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**striptags** | Option<**String**> | Determines if fields include HTML or not ('Hospitalized by <a href=...>user</a>' vs 'Hospitalized by user'). |  |[default to true]

### Return type

[**models::ForumPostsResponse**](ForumPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_thread_id_thread_get

> models::ForumThreadResponse forum_thread_id_thread_get(key, thread_id)
Get specific thread details

Contains details of a thread including topic content and poll (if any).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**thread_id** | **i32** | Thread id | [required] |

### Return type

[**models::ForumThreadResponse**](ForumThreadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_threads_get

> models::ForumThreadsResponse forum_threads_get(key, limit, sort, to, from)
Get threads across all forum categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | API key (Public) | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**sort** | Option<**String**> | Sorted by the greatest of first_post_time and last_post_time timestamps |  |[default to DESC]
**to** | Option<**i32**> | Returns threads created before this timestamp |  |
**from** | Option<**i32**> | Returns threads created after this timestamp |  |

### Return type

[**models::ForumThreadsResponse**](ForumThreadsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forum_timestamp_get

> models::TimestampResponse forum_timestamp_get(key)
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

