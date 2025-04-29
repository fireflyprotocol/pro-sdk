# AccountFundingRateHistoryData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**paymentAmountE9** | **string** | Payment amount in e9 format. | [default to undefined]
**positionSide** | [**PositionSide**](PositionSide.md) |  | [default to undefined]
**rateE9** | **string** | Funding rate value (e9 format). | [default to undefined]
**symbol** | **string** | Market address. | [default to undefined]
**executedAtMillis** | **number** | Execution timestamp in milliseconds since Unix epoch. | [default to undefined]
**computedAtMillis** | **number** | Computed timestamp in milliseconds since Unix epoch. | [default to undefined]

## Example

```typescript
import { AccountFundingRateHistoryData } from '@bluefin/api-client';

const instance: AccountFundingRateHistoryData = {
    paymentAmountE9,
    positionSide,
    rateE9,
    symbol,
    executedAtMillis,
    computedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
