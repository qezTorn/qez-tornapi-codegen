# ForumThreadUserExtended

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**forum_id** | Option<**i32**> |  | [optional]
**posts** | Option<**i32**> |  | [optional]
**rating** | Option<**i32**> |  | [optional]
**views** | Option<**i32**> | Total number of times players have opened this thread. | [optional]
**author** | Option<[**models::ForumThreadAuthor**](ForumThreadAuthor.md)> |  | [optional]
**last_poster** | Option<[**models::ForumThreadAuthor**](ForumThreadAuthor.md)> |  | [optional]
**first_post_time** | Option<**i32**> |  | [optional]
**last_post_time** | Option<**i32**> |  | [optional]
**has_poll** | Option<**bool**> |  | [optional]
**is_locked** | Option<**bool**> |  | [optional]
**is_sticky** | Option<**bool**> |  | [optional]
**new_posts** | Option<**i32**> | Available only when requesting data for yourself (no id or your id) with at least 'Minimal' access type key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


