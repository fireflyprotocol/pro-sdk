# WithdrawRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assetSymbol** | **string** | Asset symbol of the withdrawn asset | [default to undefined]
**accountAddress** | **string** | The Account Address from which to withdraw assets | [default to undefined]
**amountE9** | **string** | The amount in e9 of the asset that the User will withdraw from their account | [default to undefined]
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
