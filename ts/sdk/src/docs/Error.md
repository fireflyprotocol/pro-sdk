# ModelError


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errorCode** | **string** | A code representing the type of error. | [optional] [default to undefined]
**message** | **string** | A human-readable message describing the error. | [default to undefined]
**details** | **{ [key: string]: any | undefined; }** | Additional structured details about the error. | [optional] [default to undefined]

## Example

```typescript
import { ModelError } from '@bluefin/api-client';

const instance: ModelError = {
    errorCode,
    message,
    details,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
