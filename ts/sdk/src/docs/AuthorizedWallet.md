# AuthorizedWallet


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **string** | The address of the authorized wallet. | [default to undefined]
**alias** | **string** | The alias of the authorized wallet. | [optional] [default to undefined]
**authorizedAtMillis** | **number** | The timestamp in milliseconds when the wallet was authorized. | [default to undefined]

## Example

```typescript
import { AuthorizedWallet } from '@bluefin/api-client';

const instance: AuthorizedWallet = {
    address,
    alias,
    authorizedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
