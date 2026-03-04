# AccountValueHistoryData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestampMillis** | **number** | Timestamp in milliseconds since Unix epoch. | [default to undefined]
**unrealizedPnlE9** | **string** | Unrealized PnL at this timestamp (e9 format). | [default to undefined]
**valueE9** | **string** | Account value at this timestamp (e9 format). | [default to undefined]
**accumulatedPnlE9** | **string** | Accumulated PnL at this timestamp (e9 format). This is the cumulative sum of all period PnLs up to this point. | [default to undefined]

## Example

```typescript
import { AccountValueHistoryData } from '@bluefin/api-client';

const instance: AccountValueHistoryData = {
    timestampMillis,
    unrealizedPnlE9,
    valueE9,
    accumulatedPnlE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
