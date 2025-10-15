# AffiliateOverview


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | The user\&#39;s wallet address | [default to undefined]
**name** | **string** | Name of the affiliate | [optional] [default to undefined]
**perpsRefereeEarningsE9** | **string** | Referee earnings from perps trading (e9 format) | [default to undefined]
**spotLPRefereeEarningsE9** | **string** | Referee earnings from spot LP (e9 format) | [default to undefined]
**lendingRefereeEarningsE9** | **string** | Referee earnings from lending (e9 format) | [default to undefined]
**emberRefereeEarningsE9** | **string** | Referee earnings from ember (e9 format) | [default to undefined]
**perpsReferralEarningsE9** | **string** | Referral earnings from perps trading (e9 format) | [default to undefined]
**spotLPReferralEarningsE9** | **string** | Referral earnings from spot LP (e9 format) | [default to undefined]
**lendingReferralEarningsE9** | **string** | Referral earnings from lending (e9 format) | [default to undefined]
**perpsTotalEarningsE9** | **string** | Total earnings from perps trading (e9 format) | [default to undefined]
**spotLPTotalEarningsE9** | **string** | Total earnings from spot LP (e9 format) | [default to undefined]
**lendingTotalEarningsE9** | **string** | Total earnings from lending (e9 format) | [default to undefined]
**emberTotalEarningsE9** | **string** | Total earnings from ember (e9 format) | [default to undefined]
**totalReferralEarningsE9** | **string** | Total earnings from referrals (e9 format) | [default to undefined]
**totalRefereeEarningsE9** | **string** | Total earnings from referee activities (e9 format) | [default to undefined]
**totalEarningsE9** | **string** | Total earnings combining referrals and referee activities (e9 format) | [default to undefined]

## Example

```typescript
import { AffiliateOverview } from '@bluefin/api-client';

const instance: AffiliateOverview = {
    userAddress,
    name,
    perpsRefereeEarningsE9,
    spotLPRefereeEarningsE9,
    lendingRefereeEarningsE9,
    emberRefereeEarningsE9,
    perpsReferralEarningsE9,
    spotLPReferralEarningsE9,
    lendingReferralEarningsE9,
    perpsTotalEarningsE9,
    spotLPTotalEarningsE9,
    lendingTotalEarningsE9,
    emberTotalEarningsE9,
    totalReferralEarningsE9,
    totalRefereeEarningsE9,
    totalEarningsE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
