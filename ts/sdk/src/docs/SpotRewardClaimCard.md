# SpotRewardClaimCard


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** |  | [default to undefined]
**poolId** | **string** | On-chain pool object address. | [default to undefined]
**poolName** | **string** |  | [default to undefined]
**claimTimestamp** | **number** | Claim time in epoch-ms. | [default to undefined]
**txDigest** | **string** |  | [default to undefined]
**rewards** | [**Array&lt;SpotRewardClaimEntry&gt;**](SpotRewardClaimEntry.md) |  | [default to undefined]

## Example

```typescript
import { SpotRewardClaimCard } from '@bluefin/api-client';

const instance: SpotRewardClaimCard = {
    userAddress,
    poolId,
    poolName,
    claimTimestamp,
    txDigest,
    rewards,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
