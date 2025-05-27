# MarketStreamMessage

A market stream message containing an event type and a payload. The payload structure depends on the event type. 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | [**MarketEventType**](MarketEventType.md) |  | [default to undefined]
**payload** | [**MarketStreamMessagePayload**](MarketStreamMessagePayload.md) |  | [default to undefined]

## Example

```typescript
import { MarketStreamMessage } from '@bluefin/api-client';

const instance: MarketStreamMessage = {
    event,
    payload,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
