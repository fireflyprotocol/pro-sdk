# AccountStreamMessage

Account stream message for account-related events. The payload depends on the event type. 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | [**AccountEventType**](AccountEventType.md) |  | 
**reason** | [**AccountEventReason**](AccountEventReason.md) |  | 
**payload** | [**AccountStreamMessagePayload**](AccountStreamMessagePayload.md) |  | 

## Example

```python
from openapi_client.models.account_stream_message import AccountStreamMessage

# TODO update the JSON string below
json = "{}"
# create an instance of AccountStreamMessage from a JSON string
account_stream_message_instance = AccountStreamMessage.from_json(json)
# print the JSON string representation of the object
print(AccountStreamMessage.to_json())

# convert the object into a dict
account_stream_message_dict = account_stream_message_instance.to_dict()
# create an instance of AccountStreamMessage from a dict
account_stream_message_from_dict = AccountStreamMessage.from_dict(account_stream_message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


