# ActiveOrderUpdate

Information about an order update.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**orderHash** | **string** | The unique hash of the order. | [default to undefined]
**clientOrderId** | **string** | The client-provided order ID. | [optional] [default to undefined]
**symbol** | **string** | The symbol of the market. | [default to undefined]
**accountAddress** | **string** | The address of the account. | [default to undefined]
**priceE9** | **string** | The price of the order in scientific notation with 9 decimal places. | [default to undefined]
**quantityE9** | **string** | The quantity of the order in scientific notation with 9 decimal places. | [default to undefined]
**filledQuantityE9** | **string** | The filled quantity of the order in scientific notation with 9 decimal places. | [default to undefined]
**side** | [**TradeSide**](TradeSide.md) |  | [default to undefined]
**leverageE9** | **string** | The leverage of the order in scientific notation with 9 decimal places. | [default to undefined]
**isIsolated** | **boolean** | Indicates if the order is isolated. | [default to undefined]
**salt** | **string** | A unique salt for the order. | [default to undefined]
**expiresAtMillis** | **number** | The expiration timestamp of the order in milliseconds. | [default to undefined]
**signedAtMillis** | **number** | The signing timestamp of the order in milliseconds. | [default to undefined]
**type** | [**OrderType**](OrderType.md) |  | [default to undefined]
**reduceOnly** | **boolean** | Indicates if the order is reduce-only. | [default to undefined]
**postOnly** | **boolean** | Indicates if the order is post-only. | [default to undefined]
**timeInForce** | [**OrderTimeInForce**](OrderTimeInForce.md) |  | [default to undefined]
**triggerPriceE9** | **string** | The trigger price for stop-limit or stop-market orders. | [optional] [default to undefined]
**status** | [**OrderStatus**](OrderStatus.md) |  | [default to undefined]
**selfTradePreventionType** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [default to undefined]
**createdAtMillis** | **number** | The timestamp when the order was placed, in milliseconds. | [default to undefined]
**updatedAtMillis** | **number** | The timestamp of the last update of the order in milliseconds. | [default to undefined]

## Example

```typescript
import { ActiveOrderUpdate } from '@bluefin/api-client';

const instance: ActiveOrderUpdate = {
    orderHash,
    clientOrderId,
    symbol,
    accountAddress,
    priceE9,
    quantityE9,
    filledQuantityE9,
    side,
    leverageE9,
    isIsolated,
    salt,
    expiresAtMillis,
    signedAtMillis,
    type,
    reduceOnly,
    postOnly,
    timeInForce,
    triggerPriceE9,
    status,
    selfTradePreventionType,
    createdAtMillis,
    updatedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
