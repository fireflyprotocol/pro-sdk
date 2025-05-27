# EpochConfigs


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaignName** | **string** | The name of the campaign. | [default to undefined]
**epochDuration** | **number** | Duration of the epoch in milliseconds. | [default to undefined]
**suiRewardsAllocation** | **string** | Allocation of Sui token rewards in the epoch (e9 format). | [default to undefined]
**blueRewardsAllocation** | **string** | Allocation of Blue token rewards in the epoch (e9 format). | [default to undefined]
**walRewardsAllocation** | **string** | Allocation of wal token rewards in the epoch (e9 format) | [default to undefined]
**config** | **{ [key: string]: any | undefined; }** | Object to add custom configurations for campaigns. | [default to undefined]

## Example

```typescript
import { EpochConfigs } from '@bluefin/api-client';

const instance: EpochConfigs = {
    campaignName,
    epochDuration,
    suiRewardsAllocation,
    blueRewardsAllocation,
    walRewardsAllocation,
    config,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
