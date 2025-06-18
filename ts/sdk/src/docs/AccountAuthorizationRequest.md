# AccountAuthorizationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signedFields** | [**AccountAuthorizationRequestSignedFields**](AccountAuthorizationRequestSignedFields.md) |  | [default to undefined]
**signature** | **string** | The signature of the request, encoded from the signedFields | [default to undefined]
**alias** | **string** | The (optional) alias of the account that is being authorized or deauthorized | [optional] [default to undefined]

## Example

```typescript
import { AccountAuthorizationRequest } from '@bluefin/api-client';

const instance: AccountAuthorizationRequest = {
    signedFields,
    signature,
    alias,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
