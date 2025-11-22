# SigPayload


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | **string** | Target address for the claim. | [default to undefined]
**receiver** | **string** | Receiver address for the claim. | [default to undefined]
**amount** | **string** | Amount to be claimed. | [default to undefined]
**expiry** | **string** | Expiry timestamp for the claim. | [default to undefined]
**nonce** | **string** | Nonce for the claim. | [default to undefined]
**type** | **number** | Type identifier for the claim. | [default to undefined]

## Example

```typescript
import { SigPayload } from '@bluefin/api-client';

const instance: SigPayload = {
    target,
    receiver,
    amount,
    expiry,
    nonce,
    type,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
