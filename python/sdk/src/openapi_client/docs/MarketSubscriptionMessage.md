# MarketSubscriptionMessage

Subscription message for market data streams.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | [**SubscriptionType**](SubscriptionType.md) |  | 
**data_streams** | [**List[MarketSubscriptionStreams]**](MarketSubscriptionStreams.md) | List of market data streams to subscribe or unsubscribe from. | 

## Example

```python
from openapi_client.models.market_subscription_message import MarketSubscriptionMessage

# TODO update the JSON string below
json = "{}"
# create an instance of MarketSubscriptionMessage from a JSON string
market_subscription_message_instance = MarketSubscriptionMessage.from_json(json)
# print the JSON string representation of the object
print(MarketSubscriptionMessage.to_json())

# convert the object into a dict
market_subscription_message_dict = market_subscription_message_instance.to_dict()
# create an instance of MarketSubscriptionMessage from a dict
market_subscription_message_from_dict = MarketSubscriptionMessage.from_dict(market_subscription_message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


