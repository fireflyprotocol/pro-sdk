# KoraUserBonusResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User wallet address | 
**epoch_number** | **int** | Epoch number for the bonus | 
**bonus_points** | **str** | Total bonus points earned | 
**bonus_cc_rewards** | **str** | Total bonus CC rewards earned | 
**criteria** | **str** | Criteria for earning the bonus | [optional] 

## Example

```python
from openapi_client.models.kora_user_bonus_response import KoraUserBonusResponse

# TODO update the JSON string below
json = "{}"
# create an instance of KoraUserBonusResponse from a JSON string
kora_user_bonus_response_instance = KoraUserBonusResponse.from_json(json)
# print the JSON string representation of the object
print(KoraUserBonusResponse.to_json())

# convert the object into a dict
kora_user_bonus_response_dict = kora_user_bonus_response_instance.to_dict()
# create an instance of KoraUserBonusResponse from a dict
kora_user_bonus_response_from_dict = KoraUserBonusResponse.from_dict(kora_user_bonus_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


