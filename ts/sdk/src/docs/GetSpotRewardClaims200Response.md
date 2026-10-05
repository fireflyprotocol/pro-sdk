# GetSpotRewardClaims200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**Array&lt;SpotRewardClaimCard&gt;**](SpotRewardClaimCard.md) |  | [default to undefined]
**nextCursor** | **number** | claimTimestamp to pass as the next cursor; -1 when there are no rows. | [default to undefined]
**isMoreDataAvailable** | **boolean** |  | [default to undefined]

## Example

```typescript
import { GetSpotRewardClaims200Response } from '@bluefin/api-client';

const instance: GetSpotRewardClaims200Response = {
    data,
    nextCursor,
    isMoreDataAvailable,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
