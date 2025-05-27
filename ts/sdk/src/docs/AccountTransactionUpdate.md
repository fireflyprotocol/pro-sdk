# AccountTransactionUpdate

Details about a transaction in the account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the market. | [optional] [default to undefined]
**transactionType** | [**TransactionType**](TransactionType.md) |  | [default to undefined]
**amountE9** | **string** | The amount of the transaction in scientific notation with 9 decimal places. | [default to undefined]
**assetSymbol** | **string** | The symbol of the asset. | [optional] [default to undefined]
**tradeId** | **string** | The trade ID associated with the transaction. | [optional] [default to undefined]
**executedAtMillis** | **number** | The timestamp when the transaction was executed in milliseconds. | [default to undefined]

## Example

```typescript
import { AccountTransactionUpdate } from '@bluefin/api-client';

const instance: AccountTransactionUpdate = {
    symbol,
    transactionType,
    amountE9,
    assetSymbol,
    tradeId,
    executedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
