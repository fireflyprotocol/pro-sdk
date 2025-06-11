# FeeConfigs

Map of various fee-related configurations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**referralPerpsFeeE9** | **string** | Earnings from referral perps fees (e9 format) | [optional] [default to undefined]
**subaffiliatePerpsEarningsE9** | **string** | Earnings from subaffiliate perps (e9 format) | [optional] [default to undefined]
**spotLPFeeE9** | **string** | Earnings from spot LP fees (e9 format) | [optional] [default to undefined]
**referralSpotLPFeeE9** | **string** | Earnings from referral spot LP fees (e9 format) | [optional] [default to undefined]
**referralLendingRewardsE9** | **string** | Earnings from referral lending rewards (e9 format) | [optional] [default to undefined]
**perpsFeeCashbackE9** | **string** | Cashback from perps fees (e9 format) | [optional] [default to undefined]

## Example

```typescript
import { FeeConfigs } from '@bluefin/api-client';

const instance: FeeConfigs = {
    referralPerpsFeeE9,
    subaffiliatePerpsEarningsE9,
    spotLPFeeE9,
    referralSpotLPFeeE9,
    referralLendingRewardsE9,
    perpsFeeCashbackE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
