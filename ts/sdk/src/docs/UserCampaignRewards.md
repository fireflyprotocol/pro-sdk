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
**blueRewardsE9** | **string** | Total blue-perp token rewards earned in the epoch (e9 format). | [default to undefined]
**suiRewardsE9** | **string** | Total sui-perp token rewards earned in the epoch (e9 format). | [default to undefined]
**walRewardsE9** | **string** | Total wal-perp rewards earned in the epoch (e9 format). | [default to undefined]
**cashRewardsE9** | **string** | Total cash rewards earned in the epoch (e9 format). | [default to undefined]
**userFeePaidE9** | **string** | Total user fee paid in the epoch (e9 format). | [default to undefined]
**intervalStartDate** | **number** | Time in milliseconds for interval start date. | [default to undefined]
**intervalEndDate** | **number** | Time in milliseconds for interval end date. | [default to undefined]
**isDisbursed** | **boolean** | Indicates if the rewards have been disbursed. | [default to undefined]
**txnDigest** | **string** | Transaction digest of the disbursement. | [default to undefined]
**claimSignature** | [**Array&lt;ClaimSignatureItem&gt;**](ClaimSignatureItem.md) | Array of claim signatures for different reward types. | [optional] [default to undefined]

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
    blueRewardsE9,
    suiRewardsE9,
    walRewardsE9,
    cashRewardsE9,
    userFeePaidE9,
    intervalStartDate,
    intervalEndDate,
    isDisbursed,
    txnDigest,
    claimSignature,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
