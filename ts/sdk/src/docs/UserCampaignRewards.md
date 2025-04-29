# UserCampaignRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data. | [default to undefined]
**campaignName** | **string** | Name of the campaign. | [default to undefined]
**epochNumber** | **number** | Epoch number for the rewards earned data. | [default to undefined]
**intervalNumber** | **number** | Interval number for the rewards earned data. | [default to undefined]
**marketAddress** | **string** | Market MarketAddress. | [default to undefined]
**status** | **string** |  | [default to undefined]
**blueRewards** | **string** | Total Blue token rewards earned in the epoch (e9 format). | [default to undefined]
**suiRewards** | **string** | Total Sui token rewards earned in the epoch (e9 format). | [default to undefined]
**cashRewards** | **string** | Total cash rewards earned in the epoch (e9 format). | [default to undefined]

## Example

```typescript
import { UserCampaignRewards } from '@bluefin/api-client';

const instance: UserCampaignRewards = {
    userAddress,
    campaignName,
    epochNumber,
    intervalNumber,
    marketAddress,
    status,
    blueRewards,
    suiRewards,
    cashRewards,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
