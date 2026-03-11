# OpenIDConfigurationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer** | **string** |  | [default to undefined]
**authorizationEndpoint** | **string** |  | [default to undefined]
**jwksUri** | **string** |  | [default to undefined]
**tokenEndpoint** | **string** |  | [default to undefined]
**responseTypesSupported** | **Array&lt;string&gt;** |  | [default to undefined]
**idTokenSigningAlgValuesSupported** | **Array&lt;string&gt;** |  | [default to undefined]
**subjectTypesSupported** | **Array&lt;string&gt;** |  | [default to undefined]
**scopesSupported** | **Array&lt;string&gt;** |  | [default to undefined]

## Example

```typescript
import { OpenIDConfigurationResponse } from '@bluefin/api-client';

const instance: OpenIDConfigurationResponse = {
    issuer,
    authorizationEndpoint,
    jwksUri,
    tokenEndpoint,
    responseTypesSupported,
    idTokenSigningAlgValuesSupported,
    subjectTypesSupported,
    scopesSupported,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
