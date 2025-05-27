# RewardsSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data. | [default to undefined]
**blueRewards** | **string** | Total Blue token rewards earned by the user (e9 format). | [default to undefined]
**suiRewards** | **string** | Total Sui token rewards earned by the user (e9 format). | [default to undefined]
**walRewards** | **string** | Total wal rewards earned by the user (e9 format). | [default to undefined]

## Example

```typescript
import { RewardsSummary } from '@bluefin/api-client';

const instance: RewardsSummary = {
    userAddress,
    blueRewards,
    suiRewards,
    walRewards,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
