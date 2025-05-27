# FundingRateEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The market symbol. | [default to undefined]
**fundingTimeAtMillis** | **number** | Timestamp of the funding time in milliseconds. | [default to undefined]
**fundingRateE9** | **string** | Funding rate for the market address. | [default to undefined]

## Example

```typescript
import { FundingRateEntry } from '@bluefin/api-client';

const instance: FundingRateEntry = {
    symbol,
    fundingTimeAtMillis,
    fundingRateE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
