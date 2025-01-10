# Attack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**started** | Option<**i32**> | Attack start timestamp. | [optional]
**ended** | Option<**i32**> | Attack end timestamp. | [optional]
**attacker** | Option<[**models::AttackAttacker**](Attack_attacker.md)> |  | [optional]
**defender** | Option<[**models::AttackPlayer**](AttackPlayer.md)> |  | [optional]
**result** | Option<[**models::FactionAttackResult**](FactionAttackResult.md)> |  | [optional]
**respect_gain** | Option<**f32**> |  | [optional]
**respect_loss** | Option<**f32**> |  | [optional]
**chain** | Option<**i32**> |  | [optional]
**is_interrupted** | Option<**bool**> | This is an experimental flag which should help determine 'assist' attacks which have not contributed to the chain. For example, attacks such as where the opponent lost to someoene else before the attacker could finish the attack. This flag might not work entirely correctly, so use with caution. | [optional]
**is_stealthed** | Option<**bool**> |  | [optional]
**is_raid** | Option<**bool**> |  | [optional]
**is_ranked_war** | Option<**bool**> |  | [optional]
**modifiers** | Option<[**models::AttackModifiers**](Attack_modifiers.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


