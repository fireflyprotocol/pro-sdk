# SponsorTxResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txBytes** | **string** | Base64 encoded sponsored transaction bytes | [default to undefined]
**txDigest** | **string** | Transaction digest | [default to undefined]
**signature** | **string** | Transaction signature | [default to undefined]
**expireAtTime** | **number** | Transaction expiration time in milliseconds since Unix epoch | [default to undefined]

## Example

```typescript
import { SponsorTxResponse } from '@bluefin/api-client';

const instance: SponsorTxResponse = {
    txBytes,
    txDigest,
    signature,
    expireAtTime,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
