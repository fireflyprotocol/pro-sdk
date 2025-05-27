# OrderbookDepthResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | Market symbol. | [default to undefined]
**lastUpdateId** | **number** | Count indicating the number of changes in orderbook state. | [default to undefined]
**updatedAtMillis** | **number** | Timestamp at which the last change in orderbook state took place, in milliseconds. | [default to undefined]
**responseSentAtMillis** | **number** | The time at which the orderbook server sent the response, in milliseconds. | [default to undefined]
**bestBidPriceE9** | **string** | The best bid price on orderbook at the moment (e9 format). | [default to undefined]
**bestBidQuantityE9** | **string** | The best bid quantity on orderbook at the moment (e9 format). | [default to undefined]
**bestAskPriceE9** | **string** | The best ask price on orderbook at the moment (e9 format). | [default to undefined]
**bestAskQuantityE9** | **string** | The best ask quantity on orderbook at the moment (e9 format). | [default to undefined]
**bidsE9** | **Array&lt;Array&lt;string&gt;&gt;** | Bids to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format. | [default to undefined]
**asksE9** | **Array&lt;Array&lt;string&gt;&gt;** | Asks to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format. | [default to undefined]

## Example

```typescript
import { OrderbookDepthResponse } from '@bluefin/api-client';

const instance: OrderbookDepthResponse = {
    symbol,
    lastUpdateId,
    updatedAtMillis,
    responseSentAtMillis,
    bestBidPriceE9,
    bestBidQuantityE9,
    bestAskPriceE9,
    bestAskQuantityE9,
    bidsE9,
    asksE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
