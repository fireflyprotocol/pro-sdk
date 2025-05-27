# AffiliateOverview


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | The user\&#39;s wallet address | [default to undefined]
**name** | **string** | Name of the affiliate | [optional] [default to undefined]
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
import { AffiliateOverview } from '@bluefin/api-client';

const instance: AffiliateOverview = {
    userAddress,
    name,
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
