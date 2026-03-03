# KoraRewardsSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data | [default to undefined]
**blueRewards** | **string** | Total Blue token rewards earned | [optional] [default to undefined]
**suiRewards** | **string** | Total Sui token rewards earned | [optional] [default to undefined]
**walRewards** | **string** | Total wal rewards earned | [optional] [default to undefined]
**ccRewards** | **string** | Total CC token rewards earned across all Kora campaigns | [optional] [default to undefined]
**koraPoints** | **string** | Total Kora points earned across all Kora campaigns | [optional] [default to undefined]

## Example

```typescript
import { KoraRewardsSummary } from '@bluefin/api-client';

const instance: KoraRewardsSummary = {
    userAddress,
    blueRewards,
    suiRewards,
    walRewards,
    ccRewards,
    koraPoints,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
