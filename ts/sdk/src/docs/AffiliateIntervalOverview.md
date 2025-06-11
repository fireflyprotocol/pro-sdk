# AffiliateIntervalOverview


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | The user\&#39;s wallet address | [default to undefined]
**name** | **string** | Name of the affiliate | [optional] [default to undefined]
**intervalNumber** | **number** | The interval number for the affiliate\&#39;s earnings data | [default to undefined]
**intervalStartDate** | **number** | Start date of the interval in milliseconds | [default to undefined]
**intervalEndDate** | **number** | End date of the interval in milliseconds | [default to undefined]
**referredSince** | **string** | Date when the user was referred | [default to undefined]
**perpsRefereeEarningsE9** | **string** | Referee earnings from perps trading (e9 format) | [default to undefined]
**spotLPRefereeEarningsE9** | **string** | Referee earnings from spot LP (e9 format) | [default to undefined]
**lendingRefereeEarningsE9** | **string** | Referee earnings from lending (e9 format) | [default to undefined]
**perpsReferralEarningsE9** | **string** | Referral earnings from perps trading (e9 format) | [default to undefined]
**spotLPReferralEarningsE9** | **string** | Referral earnings from spot LP (e9 format) | [default to undefined]
**lendingReferralEarningsE9** | **string** | Referral earnings from lending (e9 format) | [default to undefined]
**perpsTotalEarningsE9** | **string** | Total earnings from perps trading (e9 format) | [default to undefined]
**spotLPTotalEarningsE9** | **string** | Total earnings from spot LP (e9 format) | [default to undefined]
**lendingTotalEarningsE9** | **string** | Total earnings from lending (e9 format) | [default to undefined]
**totalReferralEarningsE9** | **string** | Total earnings from referrals (e9 format) | [default to undefined]
**totalRefereeEarningsE9** | **string** | Total earnings from referee activities (e9 format) | [default to undefined]
**totalEarningsE9** | **string** | Total earnings combining referrals and referee activities (e9 format) | [default to undefined]

## Example

```typescript
import { AffiliateIntervalOverview } from '@bluefin/api-client';

const instance: AffiliateIntervalOverview = {
    userAddress,
    name,
    intervalNumber,
    intervalStartDate,
    intervalEndDate,
    referredSince,
    perpsRefereeEarningsE9,
    spotLPRefereeEarningsE9,
    lendingRefereeEarningsE9,
    perpsReferralEarningsE9,
    spotLPReferralEarningsE9,
    lendingReferralEarningsE9,
    perpsTotalEarningsE9,
    spotLPTotalEarningsE9,
    lendingTotalEarningsE9,
    totalReferralEarningsE9,
    totalRefereeEarningsE9,
    totalEarningsE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
