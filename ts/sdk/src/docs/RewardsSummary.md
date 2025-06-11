# RewardsSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data. | [default to undefined]
**blueRewardsE9** | **string** | Total Blue token rewards earned by the user (e9 format). | [default to undefined]
**suiRewardsE9** | **string** | Total Sui token rewards earned by the user (e9 format). | [default to undefined]
**walRewardsE9** | **string** | Total wal rewards earned by the user (e9 format). | [default to undefined]

## Example

```typescript
import { RewardsSummary } from '@bluefin/api-client';

const instance: RewardsSummary = {
    userAddress,
    blueRewardsE9,
    suiRewardsE9,
    walRewardsE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
