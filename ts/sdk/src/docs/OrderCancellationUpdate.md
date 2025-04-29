# OrderCancellationUpdate

Details of an order cancellation.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**orderHash** | **string** | The unique hash of the order. | [default to undefined]
**clientOrderId** | **string** | The client-provided order ID. | [optional] [default to undefined]
**symbol** | **string** | The symbol of the market. | [default to undefined]
**accountAddress** | **string** | The address of the account. | [default to undefined]
**createdAtMillis** | **number** | The timestamp of the order creation in milliseconds. | [default to undefined]
**cancellationReason** | [**OrderCancelReason**](OrderCancelReason.md) |  | [default to undefined]
**failureToCancelReason** | [**OrderCancellationFailureReason**](OrderCancellationFailureReason.md) |  | [optional] [default to undefined]
**remainingQuantityE9** | **string** | The remaining quantity of the order. | [default to undefined]

## Example

```typescript
import { OrderCancellationUpdate } from '@bluefin/api-client';

const instance: OrderCancellationUpdate = {
    orderHash,
    clientOrderId,
    symbol,
    accountAddress,
    createdAtMillis,
    cancellationReason,
    failureToCancelReason,
    remainingQuantityE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
