# IntervalRewards


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User address for the rewards earned data. | [default to undefined]
**status** | **string** |  | [default to undefined]
**blueRewards** | **string** | Total Blue token rewards earned in the interval (e9 format). | [default to undefined]
**suiRewards** | **string** | Total Sui token rewards earned in the interval (e9 format). | [default to undefined]
**walRewards** | **string** | Total wal rewards earned in the interval (e9 format). | [default to undefined]
**intervalNumber** | **number** | Interval Id of the interval for the rewards earned data. | [default to undefined]

## Example

```typescript
import { IntervalRewards } from '@bluefin/api-client';

const instance: IntervalRewards = {
    userAddress,
    status,
    blueRewards,
    suiRewards,
    walRewards,
    intervalNumber,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
