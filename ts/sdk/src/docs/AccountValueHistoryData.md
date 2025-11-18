# AccountValueHistoryData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestampMillis** | **number** | Timestamp in milliseconds since Unix epoch. | [default to undefined]
**unrealizedPnlE9** | **string** | Unrealized PnL at this timestamp (e9 format). | [default to undefined]
**valueE9** | **string** | Account value at this timestamp (e9 format). | [default to undefined]

## Example

```typescript
import { AccountValueHistoryData } from '@bluefin/api-client';

const instance: AccountValueHistoryData = {
    timestampMillis,
    unrealizedPnlE9,
    valueE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
