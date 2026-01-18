# CreateOrderRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signedFields** | [**CreateOrderRequestSignedFields**](CreateOrderRequestSignedFields.md) |  | [default to undefined]
**signature** | **string** | The signature of the request, encoded from the signedFields | [default to undefined]
**clientOrderId** | **string** | The client-defined unique identifier of this order used for lookup. This should always be unique; however, the server will not gurantee this or impose any checks. | [optional] [default to undefined]
**type** | [**OrderType**](OrderType.md) |  | [default to undefined]
**reduceOnly** | **boolean** | Is this order to only reduce a position? Default false | [default to undefined]
**postOnly** | **boolean** | If set to TRUE, the order can only be a maker order | [optional] [default to false]
**timeInForce** | [**OrderTimeInForce**](OrderTimeInForce.md) | Omit or set to null for market orders; otherwise, choose a valid time-in-force value. GTT: Good Til Time  IOC: Immediate Or Cancel  FOK: Fill Or Kill  | [optional] [default to undefined]
**triggerPriceE9** | **string** | Trigger price in base e9 for stop orders. This should always be a number | [optional] [default to undefined]
**selfTradePreventionType** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [optional] [default to undefined]
**twapConfig** | [**CreateOrderRequestTwapConfig**](CreateOrderRequestTwapConfig.md) |  | [optional] [default to undefined]

## Example

```typescript
import { CreateOrderRequest } from '@bluefin/api-client';

const instance: CreateOrderRequest = {
    signedFields,
    signature,
    clientOrderId,
    type,
    reduceOnly,
    postOnly,
    timeInForce,
    triggerPriceE9,
    selfTradePreventionType,
    twapConfig,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
