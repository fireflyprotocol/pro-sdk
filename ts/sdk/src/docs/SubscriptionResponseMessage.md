# SubscriptionResponseMessage

Response message indicating the success or failure of a subscription operation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **boolean** | Indicates if the subscription operation was successful. | [default to undefined]
**message** | **string** | Additional information about the subscription operation. | [default to undefined]

## Example

```typescript
import { SubscriptionResponseMessage } from '@bluefin/api-client';

const instance: SubscriptionResponseMessage = {
    success,
    message,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
