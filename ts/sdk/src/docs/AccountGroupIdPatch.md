# AccountGroupIdPatch


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accountAddress** | **string** | The address of the account to update. | [default to undefined]
**groupId** | **string** | The new group to assign the account to.  If not present, the account will be removed from any group.  | [optional] [default to undefined]

## Example

```typescript
import { AccountGroupIdPatch } from '@bluefin/api-client';

const instance: AccountGroupIdPatch = {
    accountAddress,
    groupId,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
