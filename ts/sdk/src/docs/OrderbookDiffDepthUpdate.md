# OrderbookDiffDepthUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updatedAtMillis** | **number** | The timestamp of the orderbook update. | [default to undefined]
**symbol** | **string** | The symbol of the market for the orderbook update. | [default to undefined]
**bidsE9** | **Array&lt;Array&lt;string&gt;&gt;** |  | [default to undefined]
**asksE9** | **Array&lt;Array&lt;string&gt;&gt;** |  | [default to undefined]
**firstUpdateId** | **number** | The ID of the first update in this batch. | [default to undefined]
**lastUpdateId** | **number** | The ID of the last update in this batch. | [default to undefined]

## Example

```typescript
import { OrderbookDiffDepthUpdate } from '@bluefin/api-client';

const instance: OrderbookDiffDepthUpdate = {
    updatedAtMillis,
    symbol,
    bidsE9,
    asksE9,
    firstUpdateId,
    lastUpdateId,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
