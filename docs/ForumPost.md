# ForumPost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**thread_id** | Option<**i32**> |  | [optional]
**author** | Option<[**models::ForumThreadAuthor**](ForumThreadAuthor.md)> |  | [optional]
**is_legacy** | Option<**bool**> | Indicates whether post was made using the old formatting engine which doesn't use HTML. | [optional]
**is_topic** | Option<**bool**> |  | [optional]
**is_edited** | Option<**bool**> |  | [optional]
**is_pinned** | Option<**bool**> |  | [optional]
**created_time** | Option<**i32**> |  | [optional]
**edited_by** | Option<**i32**> |  | [optional]
**has_quote** | Option<**bool**> |  | [optional]
**quoted_post_id** | Option<**i32**> | 'quoted_post_id' is null when 'has_quote' is false. | [optional]
**content** | Option<**String**> | depending on the input 'cat' parameter, this will either return raw value (with HTML) or plain text. Legacy posts are returned as is, they can't be stripped of tags. | [optional]
**likes** | Option<**i32**> |  | [optional]
**dislikes** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


