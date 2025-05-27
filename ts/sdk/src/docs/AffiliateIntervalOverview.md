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
**perpsRefereeEarnings** | **string** | Referee earnings from perps trading (e9 format) | [default to undefined]
**spotLPRefereeEarnings** | **string** | Referee earnings from spot LP (e9 format) | [default to undefined]
**lendingRefereeEarnings** | **string** | Referee earnings from lending (e9 format) | [default to undefined]
**perpsReferralEarnings** | **string** | Referral earnings from perps trading (e9 format) | [default to undefined]
**spotLPReferralEarnings** | **string** | Referral earnings from spot LP (e9 format) | [default to undefined]
**lendingReferralEarnings** | **string** | Referral earnings from lending (e9 format) | [default to undefined]
**perpsTotalEarnings** | **string** | Total earnings from perps trading (e9 format) | [default to undefined]
**spotLPTotalEarnings** | **string** | Total earnings from spot LP (e9 format) | [default to undefined]
**lendingTotalEarnings** | **string** | Total earnings from lending (e9 format) | [default to undefined]
**totalReferralEarnings** | **string** | Total earnings from referrals (e9 format) | [default to undefined]
**totalRefereeEarnings** | **string** | Total earnings from referee activities (e9 format) | [default to undefined]
**totalEarnings** | **string** | Total earnings combining referrals and referee activities (e9 format) | [default to undefined]

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
    perpsRefereeEarnings,
    spotLPRefereeEarnings,
    lendingRefereeEarnings,
    perpsReferralEarnings,
    spotLPReferralEarnings,
    lendingReferralEarnings,
    perpsTotalEarnings,
    spotLPTotalEarnings,
    lendingTotalEarnings,
    totalReferralEarnings,
    totalRefereeEarnings,
    totalEarnings,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
