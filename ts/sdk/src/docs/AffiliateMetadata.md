# AffiliateMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | The user\&#39;s wallet address | [default to undefined]
**parentReferralCode** | **string** | The referral code of the parent affiliate | [optional] [default to undefined]
**referralCode** | **string** | The user\&#39;s referral code if approved as an affiliate | [optional] [default to undefined]
**name** | **string** | The name of the affiliate | [optional] [default to undefined]
**parentName** | **string** | The name of the parent affiliate | [optional] [default to undefined]
**fees** | [**AffiliateMetadataFees**](AffiliateMetadataFees.md) |  | [optional] [default to undefined]
**status** | **string** | Status of the affiliate application | [optional] [default to undefined]
**hasTraded** | **boolean** | Indicates whether the user has traded or not | [default to undefined]
**tier** | **string** | Tier of the affiliate | [optional] [default to undefined]

## Example

```typescript
import { AffiliateMetadata } from '@bluefin/api-client';

const instance: AffiliateMetadata = {
    userAddress,
    parentReferralCode,
    referralCode,
    name,
    parentName,
    fees,
    status,
    hasTraded,
    tier,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
