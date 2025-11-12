# AccountValueHistory


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**latestValueE9** | **string** | Latest account value (e9 format). | [default to undefined]
**valueChangeE9** | **string** | Change in value from the first to the last value in the interval (e9 format). | [default to undefined]
**valueChangePercentageE9** | **string** | Percentage change in value from the first to the last value in the interval (e9 format). | [default to undefined]
**latestUnrealizedPnlE9** | **string** | Latest unrealized PnL value (e9 format). | [default to undefined]
**unrealizedPnlChangeE9** | **string** | Change in unrealized PnL from the first to the last value in the interval (e9 format). | [default to undefined]
**unrealizedPnlChangePercentageE9** | **string** | Percentage change in unrealized PnL from the first to the last value in the interval (e9 format). | [default to undefined]
**totalValueE9** | **string** | Total value across all data points (e9 format). | [default to undefined]
**values** | [**Array&lt;AccountValueHistoryData&gt;**](AccountValueHistoryData.md) |  | [default to undefined]

## Example

```typescript
import { AccountValueHistory } from '@bluefin/api-client';

const instance: AccountValueHistory = {
    latestValueE9,
    valueChangeE9,
    valueChangePercentageE9,
    latestUnrealizedPnlE9,
    unrealizedPnlChangeE9,
    unrealizedPnlChangePercentageE9,
    totalValueE9,
    values,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
