# SubscriptionResponseMessage

Response message indicating the success or failure of a subscription operation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** | Indicates if the subscription operation was successful. | 
**message** | **str** | Additional information about the subscription operation. | 

## Example

```python
from openapi_client.models.subscription_response_message import SubscriptionResponseMessage

# TODO update the JSON string below
json = "{}"
# create an instance of SubscriptionResponseMessage from a JSON string
subscription_response_message_instance = SubscriptionResponseMessage.from_json(json)
# print the JSON string representation of the object
print(SubscriptionResponseMessage.to_json())

# convert the object into a dict
subscription_response_message_dict = subscription_response_message_instance.to_dict()
# create an instance of SubscriptionResponseMessage from a dict
subscription_response_message_from_dict = SubscriptionResponseMessage.from_dict(subscription_response_message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


