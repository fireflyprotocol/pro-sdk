# AccountPositionUpdate

Details about an account position update.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the market. | [default to undefined]
**avgEntryPriceE9** | **string** | The average entry price for the position. | [default to undefined]
**clientSetLeverageE9** | **string** | The leverage applied to the position. | [default to undefined]
**liquidationPriceE9** | **string** | The liquidation price of the position. | [default to undefined]
**markPriceE9** | **string** | The current mark price of the position. | [default to undefined]
**notionalValueE9** | **string** | The notional value of the position. | [default to undefined]
**sizeE9** | **string** | The size of the position. | [default to undefined]
**unrealizedPnlE9** | **string** | The unrealized profit and loss for the position. | [default to undefined]
**side** | [**PositionSide**](PositionSide.md) |  | [default to undefined]
**marginRequiredE9** | **string** | The margin required for the position. | [default to undefined]
**maintenanceMarginE9** | **string** | The maintenance margin required for the position. | [default to undefined]
**isIsolated** | **boolean** | Indicates if the position is isolated. | [default to undefined]
**isolatedMarginE9** | **string** | The isolated margin applied to the position. | [default to undefined]
**updatedAtMillis** | **number** | The last update time for the position in milliseconds. | [default to undefined]

## Example

```typescript
import { AccountPositionUpdate } from '@bluefin/api-client';

const instance: AccountPositionUpdate = {
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
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
