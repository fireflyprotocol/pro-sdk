# IntervalRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data. | [default to undefined]
**status** | **string** |  | [default to undefined]
**blueRewardsE9** | **string** | Total Blue token rewards earned in the interval (e9 format). | [default to undefined]
**suiRewardsE9** | **string** | Total Sui token rewards earned in the interval (e9 format). | [default to undefined]
**walRewardsE9** | **string** | Total wal rewards earned in the interval (e9 format). | [default to undefined]
**intervalNumber** | **number** | Interval Id of the interval for the rewards earned data. | [default to undefined]

## Example

```typescript
import { IntervalRewards } from '@bluefin/api-client';

const instance: IntervalRewards = {
    userAddress,
    status,
    blueRewardsE9,
    suiRewardsE9,
    walRewardsE9,
    intervalNumber,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
