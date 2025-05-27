# OrderbookPartialDepthUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updatedAtMillis** | **number** | The timestamp of the partial depth update. | [default to undefined]
**symbol** | **string** | The symbol of the market for the partial depth update. | [default to undefined]
**bidsE9** | **Array&lt;Array&lt;string&gt;&gt;** |  | [default to undefined]
**asksE9** | **Array&lt;Array&lt;string&gt;&gt;** |  | [default to undefined]
**orderbookUpdateId** | **number** | The unique identifier for the orderbook update. | [default to undefined]
**depthLevel** | **string** | The depth level of the orderbook snapshot. | [default to undefined]

## Example

```typescript
import { OrderbookPartialDepthUpdate } from '@bluefin/api-client';

const instance: OrderbookPartialDepthUpdate = {
    updatedAtMillis,
    symbol,
    bidsE9,
    asksE9,
    orderbookUpdateId,
    depthLevel,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
