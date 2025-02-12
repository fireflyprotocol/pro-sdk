# AccountMarketPreference


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**margin_type** | [**MarginTypeEnum**](MarginTypeEnum.md) |  | [optional] 
**set_leverage** | **int** | User set leverage (e.g., 10x). | [optional] 

## Example

```python
from openapi_client.models.account_market_preference import AccountMarketPreference

# TODO update the JSON string below
json = "{}"
# create an instance of AccountMarketPreference from a JSON string
account_market_preference_instance = AccountMarketPreference.from_json(json)
# print the JSON string representation of the object
print(AccountMarketPreference.to_json())

# convert the object into a dict
account_market_preference_dict = account_market_preference_instance.to_dict()
# create an instance of AccountMarketPreference from a dict
account_market_preference_from_dict = AccountMarketPreference.from_dict(account_market_preference_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


