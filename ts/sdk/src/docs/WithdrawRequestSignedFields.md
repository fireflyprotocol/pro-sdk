# WithdrawRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assetSymbol** | **string** | Asset symbol of the withdrawn asset | [default to undefined]
**accountAddress** | **string** | The Account Address from which to withdraw assets | [default to undefined]
**amountE9** | **string** | The amount in e9 of the asset that the User will withdraw from their account. Must be a whole number of the asset\&#39;s smallest on-chain unit, i.e. a multiple of 10^(9 - decimals) where decimals is the asset\&#39;s on-chain decimals (1000 for USDC, which has 6 decimals). Other amounts are rejected after the request is accepted, via a failed-command event. Round down to that multiple before submitting, including when withdrawing maxWithdrawQuantityE9. | [default to undefined]
**salt** | **string** | A uniqueness modifier for the request. This is added to guarantee uniqueness of the request. Usually a mix of timestamp and a random number | [default to undefined]
**edsId** | **string** | the ID of the external datastore for the target network | [default to undefined]
**signedAtMillis** | **number** | The timestamp in milliseconds when the HTTP Request payload has been signed | [default to undefined]

## Example

```typescript
import { WithdrawRequestSignedFields } from '@bluefin/api-client';

const instance: WithdrawRequestSignedFields = {
    assetSymbol,
    accountAddress,
    amountE9,
    salt,
    edsId,
    signedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
