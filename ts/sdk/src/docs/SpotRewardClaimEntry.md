# SpotRewardClaimEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rewardType** | **string** | Reward/fee token symbol; a fee and reward of the same symbol are merged. | [default to undefined]
**rewardedAmount** | **string** | Claimed amount in the token\&#39;s own units (decimal string, e.g. \&quot;0.61172\&quot;). | [default to undefined]

## Example

```typescript
import { SpotRewardClaimEntry } from '@bluefin/api-client';

const instance: SpotRewardClaimEntry = {
    rewardType,
    rewardedAmount,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
