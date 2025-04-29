# AdjustIsolatedMarginRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idsId** | **string** | the ID of the internal datastore for the target network | [default to undefined]
**accountAddress** | **string** | The account address of the account for which to adjust margin | [default to undefined]
**symbol** | **string** | The symbol of the isolated position for which to adjust margin | [default to undefined]
**operation** | [**AdjustMarginOperation**](AdjustMarginOperation.md) |  | [default to undefined]
**quantityE9** | **string** | The quantity of margin to adjust for the isolated position | [default to undefined]
**salt** | **string** | The random generated SALT. Should always be a number | [default to undefined]
**signedAtMillis** | **number** | The timestamp in millis at which the request was signed | [default to undefined]

## Example

```typescript
import { AdjustIsolatedMarginRequestSignedFields } from '@bluefin/api-client';

const instance: AdjustIsolatedMarginRequestSignedFields = {
    idsId,
    accountAddress,
    symbol,
    operation,
    quantityE9,
    salt,
    signedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
