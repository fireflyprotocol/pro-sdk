# Position


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | Market address. | [default to undefined]
**avgEntryPriceE9** | **string** | Average entry price determined by a simple average of all entry prices resulting in this position size (e9 format). | [default to undefined]
**clientSetLeverageE9** | **string** | Isolated position leverage (e9 format). | [default to undefined]
**liquidationPriceE9** | **string** | Liquidation price (e9 format). | [default to undefined]
**markPriceE9** | **string** | Mark price (e9 format). | [default to undefined]
**notionalValueE9** | **string** | Notional value (e9 format). | [default to undefined]
**sizeE9** | **string** | Position size (e9 format). | [default to undefined]
**unrealizedPnlE9** | **string** | Unrealized profit (e9 format). | [default to undefined]
**side** | [**PositionSide**](PositionSide.md) |  | [default to undefined]
**marginRequiredE9** | **string** | Initial margin required with current mark price (e9 format). | [default to undefined]
**maintenanceMarginE9** | **string** | Maintenance margin required with current mark price (e9 format). | [default to undefined]
**isIsolated** | **boolean** | If the position is isolated. | [default to undefined]
**isolatedMarginE9** | **string** | Margin value present if margin type is isolated (e9 format). | [default to undefined]
**updatedAtMillis** | **number** | Last update time. | [default to undefined]
**fundingRatePaymentAllTimeE9** | **string** | Total funding rate payment (e9 format). | [default to undefined]
**fundingRatePaymentSinceChangeE9** | **string** | Funding rate payment since last position change (e9 format). | [default to undefined]
**fundingRatePaymentSinceOpenedE9** | **string** | Funding rate payment since position opened (e9 format). | [default to undefined]

## Example

```typescript
import { Position } from '@bluefin/api-client';

const instance: Position = {
    symbol,
    avgEntryPriceE9,
    clientSetLeverageE9,
    liquidationPriceE9,
    markPriceE9,
    notionalValueE9,
    sizeE9,
    unrealizedPnlE9,
    side,
    marginRequiredE9,
    maintenanceMarginE9,
    isIsolated,
    isolatedMarginE9,
    updatedAtMillis,
    fundingRatePaymentAllTimeE9,
    fundingRatePaymentSinceChangeE9,
    fundingRatePaymentSinceOpenedE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
