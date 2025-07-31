# EpochConfigs


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaignName** | **string** | The name of the campaign. | [default to undefined]
**epochDuration** | **number** | Duration of the epoch in seconds. | [default to undefined]
**suiRewardsAllocationE9** | **string** | Allocation of Sui token rewards in the epoch (e9 format). | [default to undefined]
**blueRewardsAllocationE9** | **string** | Allocation of Blue token rewards in the epoch (e9 format). | [default to undefined]
**walRewardsAllocationE9** | **string** | Allocation of wal token rewards in the epoch (e9 format) | [default to undefined]
**intervalNumber** | **number** | Interval number for the epoch. | [default to undefined]
**epochNumber** | **number** | Epoch number for the epoch. | [default to undefined]
**config** | **{ [key: string]: any | undefined; }** | Object to add custom configurations for campaigns. | [default to undefined]

## Example

```typescript
import { EpochConfigs } from '@bluefin/api-client';

const instance: EpochConfigs = {
    campaignName,
    epochDuration,
    suiRewardsAllocationE9,
    blueRewardsAllocationE9,
    walRewardsAllocationE9,
    intervalNumber,
    epochNumber,
    config,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
