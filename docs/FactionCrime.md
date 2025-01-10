# FactionCrime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**difficulty** | Option<**i32**> |  | [optional]
**status** | Option<[**models::FactionCrimeStatusEnum**](FactionCrimeStatusEnum.md)> |  | [optional]
**created_at** | Option<**i32**> | The timestamp at which the crime was created. | [optional]
**initiated_at** | Option<**i32**> | Replaced by the 'planning_at' field (same value). This field will be removed in the future (early March or later). | [optional]
**planning_at** | Option<**i32**> | The timestamp at which the planning phase for the crime has begun. | [optional]
**ready_at** | Option<**i32**> | The timestamp at which the crime will be ready. | [optional]
**expired_at** | Option<**i32**> | The timestamp at which the crime will expire. | [optional]
**slots** | Option<[**Vec<models::FactionCrimeSlot>**](FactionCrimeSlot.md)> |  | [optional]
**rewards** | Option<[**models::FactionCrimeRewards**](FactionCrime_rewards.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


