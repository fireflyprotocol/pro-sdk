# AccountSubscriptionMessage

Subscription message for account data streams.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | [**SubscriptionType**](SubscriptionType.md) |  | 
**data_streams** | [**List[AccountDataStream]**](AccountDataStream.md) | List of account data streams to subscribe or unsubscribe from. | 

## Example

```python
from openapi_client.models.account_subscription_message import AccountSubscriptionMessage

# TODO update the JSON string below
json = "{}"
# create an instance of AccountSubscriptionMessage from a JSON string
account_subscription_message_instance = AccountSubscriptionMessage.from_json(json)
# print the JSON string representation of the object
print(AccountSubscriptionMessage.to_json())

# convert the object into a dict
account_subscription_message_dict = account_subscription_message_instance.to_dict()
# create an instance of AccountSubscriptionMessage from a dict
account_subscription_message_from_dict = AccountSubscriptionMessage.from_dict(account_subscription_message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


