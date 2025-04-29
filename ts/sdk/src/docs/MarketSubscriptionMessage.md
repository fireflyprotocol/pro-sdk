# MarketSubscriptionMessage

Subscription message for market data streams.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | [**SubscriptionType**](SubscriptionType.md) |  | [default to undefined]
**dataStreams** | [**Array&lt;MarketSubscriptionStreams&gt;**](MarketSubscriptionStreams.md) | List of market data streams to subscribe or unsubscribe from. | [default to undefined]

## Example

```typescript
import { MarketSubscriptionMessage } from '@bluefin/api-client';

const instance: MarketSubscriptionMessage = {
    method,
    dataStreams,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
