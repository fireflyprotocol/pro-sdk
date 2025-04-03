# RecentTradesUpdates


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trades** | [**List[Trade]**](Trade.md) |  | 

## Example

```python
from openapi_client.models.recent_trades_updates import RecentTradesUpdates

# TODO update the JSON string below
json = "{}"
# create an instance of RecentTradesUpdates from a JSON string
recent_trades_updates_instance = RecentTradesUpdates.from_json(json)
# print the JSON string representation of the object
print(RecentTradesUpdates.to_json())

# convert the object into a dict
recent_trades_updates_dict = recent_trades_updates_instance.to_dict()
# create an instance of RecentTradesUpdates from a dict
recent_trades_updates_from_dict = RecentTradesUpdates.from_dict(recent_trades_updates_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


