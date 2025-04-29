# AccountPositionLeverageUpdateRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accountAddress** | **string** | The Account Address from which to update leverage | [default to undefined]
**symbol** | **string** | Symbol of the perpetual of the positions for which to update the leverage | [default to undefined]
**leverageE9** | **string** | The leverage to set for the account positions (Must be a number in base e9) | [default to undefined]
**salt** | **string** | The random generated SALT. Should always be a number | [default to undefined]
**idsId** | **string** | the ID of the internal datastore for the target network | [default to undefined]
**signedAtMillis** | **number** | The timestamp in millis at which the request was signed | [default to undefined]

## Example

```typescript
import { AccountPositionLeverageUpdateRequestSignedFields } from '@bluefin/api-client';

const instance: AccountPositionLeverageUpdateRequestSignedFields = {
    accountAddress,
    symbol,
    leverageE9,
    salt,
    idsId,
    signedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
