# AccountSubscriptionMessage

Subscription message for account data streams.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authToken** | **string** | The authentication token for the account. | [optional] [default to undefined]
**method** | [**SubscriptionType**](SubscriptionType.md) |  | [default to undefined]
**dataStreams** | [**Array&lt;AccountDataStream&gt;**](AccountDataStream.md) | List of account data streams to subscribe or unsubscribe from. | [default to undefined]

## Example

```typescript
import { AccountSubscriptionMessage } from '@bluefin/api-client';

const instance: AccountSubscriptionMessage = {
    authToken,
    method,
    dataStreams,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
