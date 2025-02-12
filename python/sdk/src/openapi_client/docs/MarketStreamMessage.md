# MarketStreamMessage

A market stream message containing an event type and a payload. The payload structure depends on the event type. 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | [**MarketEventType**](MarketEventType.md) |  | 
**payload** | [**MarketStreamMessagePayload**](MarketStreamMessagePayload.md) |  | 

## Example

```python
from openapi_client.models.market_stream_message import MarketStreamMessage

# TODO update the JSON string below
json = "{}"
# create an instance of MarketStreamMessage from a JSON string
market_stream_message_instance = MarketStreamMessage.from_json(json)
# print the JSON string representation of the object
print(MarketStreamMessage.to_json())

# convert the object into a dict
market_stream_message_dict = market_stream_message_instance.to_dict()
# create an instance of MarketStreamMessage from a dict
market_stream_message_from_dict = MarketStreamMessage.from_dict(market_stream_message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


