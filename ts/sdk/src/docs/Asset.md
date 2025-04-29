# Asset

Details about an asset in the account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the asset. | [default to undefined]
**quantityE9** | **string** | The quantity of the asset. | [default to undefined]
**effectiveBalanceE9** | **string** | The effective balance of the asset. | [default to undefined]
**maxWithdrawQuantityE9** | **string** | The maximum quantity that can be withdrawn. | [default to undefined]
**updatedAtMillis** | **number** | The timestamp of the last update in milliseconds. | [default to undefined]

## Example

```typescript
import { Asset } from '@bluefin/api-client';

const instance: Asset = {
    symbol,
    quantityE9,
    effectiveBalanceE9,
    maxWithdrawQuantityE9,
    updatedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
