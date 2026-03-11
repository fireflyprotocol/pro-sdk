# BalanceResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User\&#39;s wallet address. | [default to undefined]
**totalPoints** | **string** | Lifetime Vera Points total. | [default to undefined]
**currentTier** | **string** | Current status tier. | [default to undefined]
**rank** | **number** | User\&#39;s rank by lifetime points (1-based). | [default to undefined]
**nextTier** | **string** | Next tier above current; null if Diamond. | [default to undefined]
**nextTierThreshold** | **string** | Points required for next tier; null if Diamond. | [default to undefined]

## Example

```typescript
import { BalanceResponse } from '@bluefin/api-client';

const instance: BalanceResponse = {
    userAddress,
    totalPoints,
    currentTier,
    rank,
    nextTier,
    nextTierThreshold,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
