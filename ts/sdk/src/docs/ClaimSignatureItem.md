# ClaimSignatureItem


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rewardType** | **string** | Type of reward for this claim signature. | [default to undefined]
**sigpayload** | [**SigPayload**](SigPayload.md) |  | [default to undefined]
**signature** | **string** | Signature for the claim. | [default to undefined]

## Example

```typescript
import { ClaimSignatureItem } from '@bluefin/api-client';

const instance: ClaimSignatureItem = {
    rewardType,
    sigpayload,
    signature,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
