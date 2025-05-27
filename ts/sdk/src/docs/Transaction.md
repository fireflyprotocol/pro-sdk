# Transaction


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **string** | Transaction ID. | [default to undefined]
**symbol** | **string** | Market address. | [optional] [default to undefined]
**type** | [**TransactionType**](TransactionType.md) |  | [default to undefined]
**amountE9** | **string** | Amount in e9 format (positive or negative). | [default to undefined]
**status** | **string** | Transaction status (SUCCESS, REJECTED). | [optional] [default to undefined]
**assetSymbol** | **string** | Asset bank address. | [default to undefined]
**tradeId** | **string** | Trade ID | [optional] [default to undefined]
**executedAtMillis** | **number** | Transaction timestamp in milliseconds since Unix epoch. | [default to undefined]

## Example

```typescript
import { Transaction } from '@bluefin/api-client';

const instance: Transaction = {
    id,
    symbol,
    type,
    amountE9,
    status,
    assetSymbol,
    tradeId,
    executedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
