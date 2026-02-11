# OpenOrderResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**orderHash** | **string** | The Order Hash, which is the default way to uniquely identify an order in the system | [default to undefined]
**clientOrderId** | **string** | The Client Order ID, which is used a unique identifier for an order, provided by the client, in case of proprietary order management systems | [optional] [default to undefined]
**symbol** | **string** | The market symbol | [default to undefined]
**accountAddress** | **string** | The account address of the order. May be an account user is authorized for. | [default to undefined]
**signerAddress** | **string** | The signer address of the order. May be an account user is authorized for. | [default to undefined]
**priceE9** | **string** | The price in base e9 of the asset to be traded. Should always be a number | [default to undefined]
**quantityE9** | **string** | The quantity in base e9 of the asset to be traded. Should always be a number | [default to undefined]
**side** | [**OrderSide**](OrderSide.md) |  | [default to undefined]
**leverageE9** | **string** | The leverage in base e9  of the order to be traded. Should always be a number | [default to undefined]
**isIsolated** | **boolean** | Is this order isolated or cross margin. Note market must be set to the same mode. | [default to false]
**salt** | **string** | The random generated SALT. Should always be a number | [default to undefined]
**expiresAtMillis** | **number** | Unix timestamp in millis at which order will expire. Defaults to 1 month for LIMIT orders if not provided | [default to undefined]
**signedAtMillis** | **number** | The timestamp in millis at which the request was signed | [default to undefined]
**type** | [**OrderType**](OrderType.md) |  | [default to undefined]
**reduceOnly** | **boolean** | Is this order to only reduce a position? Default false | [default to false]
**postOnly** | **boolean** | If set to TRUE, the order can only be a maker order | [default to false]
**timeInForce** | [**OrderTimeInForce**](OrderTimeInForce.md) |  | [default to undefined]
**triggerPriceE9** | **string** | Trigger price in base e9 for stop orders. This should always be a number | [optional] [default to undefined]
**filledQuantityE9** | **string** | The quantity in base e9 of the asset currently filled. This should always be a number | [default to undefined]
**status** | [**OrderStatus**](OrderStatus.md) |  | [default to undefined]
**selfTradePreventionType** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [default to undefined]
**twapConfig** | [**OrderTwapConfig**](OrderTwapConfig.md) |  | [optional] [default to undefined]
**orderTimeAtMillis** | **number** | The timestamp in millis when the order was opened | [default to undefined]
**updatedAtMillis** | **number** | The timestamp in millis that this order was last updated (including status updates) | [default to undefined]

## Example

```typescript
import { OpenOrderResponse } from '@bluefin/api-client';

const instance: OpenOrderResponse = {
    orderHash,
    clientOrderId,
    symbol,
    accountAddress,
    signerAddress,
    priceE9,
    quantityE9,
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
    filledQuantityE9,
    status,
    selfTradePreventionType,
    twapConfig,
    orderTimeAtMillis,
    updatedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
