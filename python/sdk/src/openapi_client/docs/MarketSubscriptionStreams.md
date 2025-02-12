# MarketSubscriptionStreams

Represents the type of market data stream and its parameters.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the market stream to subscribe to (Leave empty for TickerAll stream) | 
**streams** | [**List[MarketDataStreamName]**](MarketDataStreamName.md) |  | 

## Example

```python
from openapi_client.models.market_subscription_streams import MarketSubscriptionStreams

# TODO update the JSON string below
json = "{}"
# create an instance of MarketSubscriptionStreams from a JSON string
market_subscription_streams_instance = MarketSubscriptionStreams.from_json(json)
# print the JSON string representation of the object
print(MarketSubscriptionStreams.to_json())

# convert the object into a dict
market_subscription_streams_dict = market_subscription_streams_instance.to_dict()
# create an instance of MarketSubscriptionStreams from a dict
market_subscription_streams_from_dict = MarketSubscriptionStreams.from_dict(market_subscription_streams_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


