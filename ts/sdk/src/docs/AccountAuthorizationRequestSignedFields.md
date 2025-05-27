# AccountAuthorizationRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accountAddress** | **string** | The account address of the parent account that is authorizing/deauthorizing this account | [default to undefined]
**authorizedAccountAddress** | **string** | The address of the account that should be authorized/deauthorized | [default to undefined]
**salt** | **string** | The random generated salt. Should always be a number | [default to undefined]
**idsId** | **string** | the ID of the internal datastore for the target network | [default to undefined]
**signedAtMillis** | **number** | The timestamp when the request was signed | [default to undefined]

## Example

```typescript
import { AccountAuthorizationRequestSignedFields } from '@bluefin/api-client';

const instance: AccountAuthorizationRequestSignedFields = {
    accountAddress,
    authorizedAccountAddress,
    salt,
    idsId,
    signedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
