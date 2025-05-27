# MarketSubscriptionStreams

Represents the type of market data stream and its parameters.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the market stream to subscribe to (Leave empty for TickerAll stream) | [default to undefined]
**streams** | [**Array&lt;MarketDataStreamName&gt;**](MarketDataStreamName.md) |  | [default to undefined]

## Example

```typescript
import { MarketSubscriptionStreams } from '@bluefin/api-client';

const instance: MarketSubscriptionStreams = {
    symbol,
    streams,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
