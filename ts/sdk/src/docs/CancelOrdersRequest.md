# CancelOrdersRequest

Cancelling Orders for a specific symbol. If order hashes are not specified, all orders are canceled for this symbol

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the perpetual for which to cancel orders. | [default to undefined]
**orderHashes** | **Array&lt;string&gt;** | List of order hashes of the orders to be cancelled. All orders must belong to accountAddress. Max 10 order hashes | [optional] [default to undefined]

## Example

```typescript
import { CancelOrdersRequest } from '@bluefin/api-client';

const instance: CancelOrdersRequest = {
    symbol,
    orderHashes,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
