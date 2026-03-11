# BalanceResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | User&#39;s wallet address. | 
**total_points** | **str** | Lifetime Vera Points total. | 
**current_tier** | **str** | Current status tier. | 
**rank** | **int** | User&#39;s rank by lifetime points (1-based). | 
**next_tier** | **str** | Next tier above current; null if Diamond. | 
**next_tier_threshold** | **str** | Points required for next tier; null if Diamond. | 

## Example

```python
from openapi_client.models.balance_response import BalanceResponse

# TODO update the JSON string below
json = "{}"
# create an instance of BalanceResponse from a JSON string
balance_response_instance = BalanceResponse.from_json(json)
# print the JSON string representation of the object
print(BalanceResponse.to_json())

# convert the object into a dict
balance_response_dict = balance_response_instance.to_dict()
# create an instance of BalanceResponse from a dict
balance_response_from_dict = BalanceResponse.from_dict(balance_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


