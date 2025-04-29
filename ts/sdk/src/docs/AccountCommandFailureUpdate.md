# AccountCommandFailureUpdate

Details about a failure during an account command execution.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reason** | **string** | The reason for the failure. | [default to undefined]
**failedCommandType** | **string** | The type of command that failed. | [default to undefined]
**failedAtMillis** | **number** | The timestamp when the command failed in milliseconds. | [default to undefined]

## Example

```typescript
import { AccountCommandFailureUpdate } from '@bluefin/api-client';

const instance: AccountCommandFailureUpdate = {
    reason,
    failedCommandType,
    failedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
