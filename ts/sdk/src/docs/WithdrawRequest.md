# WithdrawRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signedFields** | [**WithdrawRequestSignedFields**](WithdrawRequestSignedFields.md) |  | [default to undefined]
**signature** | **string** | The signature of the request, encoded from the signedFields | [default to undefined]

## Example

```typescript
import { WithdrawRequest } from '@bluefin/api-client';

const instance: WithdrawRequest = {
    signedFields,
    signature,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
