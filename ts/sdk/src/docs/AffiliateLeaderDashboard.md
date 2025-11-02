# AffiliateLeaderDashboard


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | The user\&#39;s wallet address | [default to undefined]
**name** | **string** | Name of the affiliate | [optional] [default to undefined]
**parentAddress** | **string** | The parent affiliate\&#39;s wallet address | [default to undefined]
**parentName** | **string** | Name of the parent affiliate | [optional] [default to undefined]
**perpsRank** | **number** | Ranking in perps trading category | [default to undefined]
**spotRank** | **number** | Ranking in spot trading category | [default to undefined]
**lendingRank** | **number** | Ranking in lending category | [default to undefined]
**emberRank** | **number** | Ranking in ember category | [default to undefined]
**perpsTotalEarningsE9** | **string** | Total earnings from perps trading (e9 format) | [default to undefined]
**spotTotalEarningsE9** | **string** | Total earnings from spot trading (e9 format) | [default to undefined]
**lendingTotalEarningsE9** | **string** | Total earnings from lending (e9 format) | [default to undefined]

## Example

```typescript
import { AffiliateLeaderDashboard } from '@bluefin/api-client';

const instance: AffiliateLeaderDashboard = {
    userAddress,
    name,
    parentAddress,
    parentName,
    perpsRank,
    spotRank,
    lendingRank,
    emberRank,
    perpsTotalEarningsE9,
    spotTotalEarningsE9,
    lendingTotalEarningsE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
