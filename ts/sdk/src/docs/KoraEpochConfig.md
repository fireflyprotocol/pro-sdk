# KoraEpochConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaignName** | **string** | The name of the campaign | [default to undefined]
**epochDuration** | **number** | Duration of the epoch in hours | [default to undefined]
**suiRewardsAllocation** | **string** | Allocation of Sui rewards for this epoch | [optional] [default to undefined]
**blueRewardsAllocation** | **string** | Allocation of Blue rewards for this epoch | [optional] [default to undefined]
**walRewardsAllocation** | **string** | Allocation of Wal rewards for this epoch | [optional] [default to undefined]
**ccRewardsAllocation** | **string** | Allocation of CC token rewards for this epoch | [optional] [default to undefined]
**koraPointsRewardsAllocation** | **string** | Allocation of Kora points rewards for this epoch | [optional] [default to undefined]
**intervalNumber** | **number** | Interval number for the epoch | [default to undefined]
**epochNumber** | **number** | Epoch number | [default to undefined]
**config** | **{ [key: string]: any | undefined; }** | Additional campaign-specific configurations | [optional] [default to undefined]

## Example

```typescript
import { KoraEpochConfig } from '@bluefin/api-client';

const instance: KoraEpochConfig = {
    campaignName,
    epochDuration,
    suiRewardsAllocation,
    blueRewardsAllocation,
    walRewardsAllocation,
    ccRewardsAllocation,
    koraPointsRewardsAllocation,
    intervalNumber,
    epochNumber,
    config,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
