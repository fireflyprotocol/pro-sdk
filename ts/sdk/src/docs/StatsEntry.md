# StatsEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**startTimeAtMillis** | **number** | Timestamp in milliseconds when the statistics period starts. | [default to undefined]
**tvlE9** | **string** | Total value locked in the exchange in e9 format. | [default to undefined]
**totalQuoteAssetVolumeE9** | **string** | Total quote asset volume in the exchange in e9 format. | [default to undefined]
**endTimeAtMillis** | **number** | Timestamp in milliseconds when the statistics period ends. | [default to undefined]

## Example

```typescript
import { StatsEntry } from '@bluefin/api-client';

const instance: StatsEntry = {
    startTimeAtMillis,
    tvlE9,
    totalQuoteAssetVolumeE9,
    endTimeAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
