# ZKLoginZKPResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proofPoints** | [**ProofPoints**](ProofPoints.md) |  | [optional] [default to undefined]
**issBase64Details** | [**IssBase64Details**](IssBase64Details.md) |  | [optional] [default to undefined]
**headerBase64** | **string** | Base64 encoded header information. | [optional] [default to undefined]
**addressSeed** | **string** | The address seed used in the proof. | [default to undefined]

## Example

```typescript
import { ZKLoginZKPResponse } from '@bluefin/api-client';

const instance: ZKLoginZKPResponse = {
    proofPoints,
    issBase64Details,
    headerBase64,
    addressSeed,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
