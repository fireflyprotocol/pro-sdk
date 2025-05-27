# OnboardAffiliateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parentReferralCode** | **string** | Referral code of the parent affiliate | [optional] [default to undefined]
**name** | **string** | Name of the applicant | [default to undefined]
**email** | **string** | Email address of the applicant | [default to undefined]
**socialUserNames** | [**OnboardAffiliateRequestSocialUserNames**](OnboardAffiliateRequestSocialUserNames.md) |  | [optional] [default to undefined]

## Example

```typescript
import { OnboardAffiliateRequest } from '@bluefin/api-client';

const instance: OnboardAffiliateRequest = {
    parentReferralCode,
    name,
    email,
    socialUserNames,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
