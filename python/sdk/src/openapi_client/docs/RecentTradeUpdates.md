# RecentTradeUpdates


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recent_trades** | [**List[RecentTradesUpdate]**](RecentTradesUpdate.md) |  | 

## Example

```python
from openapi_client.models.recent_trade_updates import RecentTradeUpdates

# TODO update the JSON string below
json = "{}"
# create an instance of RecentTradeUpdates from a JSON string
recent_trade_updates_instance = RecentTradeUpdates.from_json(json)
# print the JSON string representation of the object
print(RecentTradeUpdates.to_json())

# convert the object into a dict
recent_trade_updates_dict = recent_trade_updates_instance.to_dict()
# create an instance of RecentTradeUpdates from a dict
recent_trade_updates_from_dict = RecentTradeUpdates.from_dict(recent_trade_updates_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


