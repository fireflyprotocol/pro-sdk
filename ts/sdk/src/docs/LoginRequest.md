# LoginRequest

User is expected to sign this payload and sends is signature in login api as header and payload itself in request body 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accountAddress** | **string** | The address of the account. | [default to undefined]
**signedAtMillis** | **number** | The timestamp in millis when the login was signed. | [default to undefined]
**audience** | **string** | The intended audience of the login request. | [default to undefined]

## Example

```typescript
import { LoginRequest } from '@bluefin/api-client';

const instance: LoginRequest = {
    accountAddress,
    signedAtMillis,
    audience,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
