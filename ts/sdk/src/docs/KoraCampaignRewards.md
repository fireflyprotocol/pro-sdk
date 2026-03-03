# KoraCampaignRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data | [default to undefined]
**campaignName** | **string** | Name of the campaign | [default to undefined]
**epochNumber** | **number** | Epoch number for the rewards earned data | [default to undefined]
**intervalNumber** | **number** | Interval number for the rewards earned data | [default to undefined]
**symbol** | **string** | Market Symbol | [optional] [default to undefined]
**status** | **string** |  | [default to undefined]
**blueRewards** | **string** | Total blue token rewards | [optional] [default to undefined]
**suiRewards** | **string** | Total sui token rewards | [optional] [default to undefined]
**walRewards** | **string** | Total wal rewards | [optional] [default to undefined]
**cashRewards** | **string** | Total cash rewards | [optional] [default to undefined]
**ccRewards** | **string** | Total CC token rewards | [optional] [default to undefined]
**koraPoints** | **string** | Total Kora points earned | [optional] [default to undefined]
**userFeePaid** | **string** | Total user fee paid | [optional] [default to undefined]
**intervalStartDate** | **number** | Time in milliseconds for interval start date | [optional] [default to undefined]
**intervalEndDate** | **number** | Time in milliseconds for interval end date | [optional] [default to undefined]
**isDisbursed** | **boolean** | Indicates if rewards have been disbursed | [optional] [default to undefined]
**txnDigest** | **string** | Transaction digest of the disbursement | [optional] [default to undefined]
**claimStatus** | **string** | Status of the claim | [optional] [default to undefined]
**bonuses** | [**Array&lt;KoraCampaignBonus&gt;**](KoraCampaignBonus.md) | List of bonuses attached to this reward entry | [optional] [default to undefined]

## Example

```typescript
import { KoraCampaignRewards } from '@bluefin/api-client';

const instance: KoraCampaignRewards = {
    userAddress,
    campaignName,
    epochNumber,
    intervalNumber,
    symbol,
    status,
    blueRewards,
    suiRewards,
    walRewards,
    cashRewards,
    ccRewards,
    koraPoints,
    userFeePaid,
    intervalStartDate,
    intervalEndDate,
    isDisbursed,
    txnDigest,
    claimStatus,
    bonuses,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
