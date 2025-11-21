# MarkAsClaimedRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intervalNumber** | **number** | The interval number | [default to undefined]
**campaignName** | **string** | The campaign name | [default to undefined]
**txnDigest** | **string** | The transaction digest of the claim | [default to undefined]
**rewardTypes** | **Array&lt;string&gt;** | The reward types to mark as claimed | [default to undefined]

## Example

```typescript
import { MarkAsClaimedRequest } from '@bluefin/api-client';

const instance: MarkAsClaimedRequest = {
    intervalNumber,
    campaignName,
    txnDigest,
    rewardTypes,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
