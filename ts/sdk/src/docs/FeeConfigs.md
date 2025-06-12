# FeeConfigs

Map of various fee-related configurations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**referralPerpsFee** | **string** | Earnings from referral perps fees | [optional] [default to undefined]
**subaffiliatePerpsEarnings** | **string** | Earnings from subaffiliate perps | [optional] [default to undefined]
**spotLPFee** | **string** | Earnings from spot LP fees | [optional] [default to undefined]
**referralSpotLPFee** | **string** | Earnings from referral spot LP fees | [optional] [default to undefined]
**referralLendingRewards** | **string** | Earnings from referral lending rewards | [optional] [default to undefined]
**perpsFeeCashback** | **string** | Cashback from perps fees | [optional] [default to undefined]

## Example

```typescript
import { FeeConfigs } from '@bluefin/api-client';

const instance: FeeConfigs = {
    referralPerpsFee,
    subaffiliatePerpsEarnings,
    spotLPFee,
    referralSpotLPFee,
    referralLendingRewards,
    perpsFeeCashback,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
