# Trade


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **string** | Trade ID | [default to undefined]
**clientOrderId** | **string** | Client order ID. | [optional] [default to undefined]
**symbol** | **string** | Market address. | [optional] [default to undefined]
**orderHash** | **string** | Order hash. | [optional] [default to undefined]
**orderType** | [**OrderType**](OrderType.md) |  | [optional] [default to undefined]
**tradeType** | [**TradeType**](TradeType.md) |  | [optional] [default to undefined]
**side** | [**TradeSide**](TradeSide.md) |  | [default to undefined]
**isMaker** | **boolean** | Indicates if the user was a maker to the trade. | [optional] [default to undefined]
**priceE9** | **string** | Trade price (e9 format). | [default to undefined]
**quantityE9** | **string** | Trade quantity (e9 format). | [default to undefined]
**quoteQuantityE9** | **string** | Quote quantity (e9 format). | [default to undefined]
**realizedPnlE9** | **string** | Realized profit and loss (e9 format). | [optional] [default to undefined]
**positionSide** | [**PositionSide**](PositionSide.md) |  | [optional] [default to undefined]
**tradingFeeE9** | **string** | Trading fee (e9 format). | [optional] [default to undefined]
**tradingFeeAsset** | **string** | Asset used for trading fee. | [optional] [default to undefined]
**gasFeeE9** | **string** | Gas fee. | [optional] [default to undefined]
**gasFeeAsset** | **string** | Asset used for gas fee. | [optional] [default to undefined]
**executedAtMillis** | **number** | Trade timestamp in milliseconds since Unix epoch. | [default to undefined]

## Example

```typescript
import { Trade } from '@bluefin/api-client';

const instance: Trade = {
    id,
    clientOrderId,
    symbol,
    orderHash,
    orderType,
    tradeType,
    side,
    isMaker,
    priceE9,
    quantityE9,
    quoteQuantityE9,
    realizedPnlE9,
    positionSide,
    tradingFeeE9,
    tradingFeeAsset,
    gasFeeE9,
    gasFeeAsset,
    executedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
