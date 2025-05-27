# UserCampaignRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data. | [default to undefined]
**campaignName** | **string** | Name of the campaign. | [default to undefined]
**epochNumber** | **number** | Epoch number for the rewards earned data. | [default to undefined]
**intervalNumber** | **number** | Interval number for the rewards earned data. | [default to undefined]
**symbol** | **string** | Market Symbol. | [default to undefined]
**status** | **string** |  | [default to undefined]
**blueRewards** | **string** | Total blue-perp token rewards earned in the epoch (e9 format). | [default to undefined]
**suiRewards** | **string** | Total sui-perp token rewards earned in the epoch (e9 format). | [default to undefined]
**walRewards** | **string** | Total wal-perp rewards earned in the epoch (e9 format). | [default to undefined]
**intervalStartDate** | **number** | Time in milliseconds for interval start date. | [default to undefined]
**intervalEndDate** | **number** | Time in milliseconds for interval end date. | [default to undefined]

## Example

```typescript
import { UserCampaignRewards } from '@bluefin/api-client';

const instance: UserCampaignRewards = {
    userAddress,
    campaignName,
    epochNumber,
    intervalNumber,
    symbol,
    status,
    blueRewards,
    suiRewards,
    walRewards,
    intervalStartDate,
    intervalEndDate,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
