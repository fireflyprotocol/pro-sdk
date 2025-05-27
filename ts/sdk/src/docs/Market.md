# Market


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | Symbol of the market. | [default to undefined]
**marketAddress** | **string** | Market address. | [default to undefined]
**status** | [**MarketStatus**](MarketStatus.md) |  | [default to undefined]
**baseAssetSymbol** | **string** | Base asset symbol. | [default to undefined]
**baseAssetName** | **string** | Base asset name. | [default to undefined]
**baseAssetDecimals** | **number** | Precision of the base asset. | [default to undefined]
**stepSizeE9** | **string** | Step size for the quantity (e9 format). | [default to undefined]
**tickSizeE9** | **string** | Price increment size (e9 format). | [default to undefined]
**minOrderQuantityE9** | **string** | Minimum order size (e9 format). | [default to undefined]
**maxLimitOrderQuantityE9** | **string** | Maximum limit order size (e9 format). | [default to undefined]
**maxMarketOrderQuantityE9** | **string** | Maximum market order size (e9 format). | [default to undefined]
**minOrderPriceE9** | **string** | Minimum order price (e9 format). | [default to undefined]
**maxOrderPriceE9** | **string** | Maximum order price (e9 format). | [default to undefined]
**maintenanceMarginRatioE9** | **string** | Maintenance margin ratio (MMR, e9 format). | [default to undefined]
**initialMarginRatioE9** | **string** | Initial margin ratio (IMR), e9 format). | [default to undefined]
**insurancePoolRatioE9** | **string** | Insurance pool ratio (e9 format). | [default to undefined]
**defaultLeverageE9** | **string** | Default leverage (e9 format). | [default to undefined]
**maxNotionalAtOpenE9** | **Array&lt;string&gt;** | Maximum notional value at current leverage. Index 0 is max notional value for leverage set to 1x, index 1 is for leverage 2x, etc... | [default to undefined]
**minTradeQuantityE9** | **string** | Minimum trade quantity allowed (e9 format). | [default to undefined]
**maxTradeQuantityE9** | **string** | Max trade quantity allowed (e9 format). | [default to undefined]
**minTradePriceE9** | **string** | Minimum trade price allowed (e9 format). | [default to undefined]
**maxTradePriceE9** | **string** | Maximum trade price allowed (e9 format). | [default to undefined]
**maxFundingRateE9** | **string** | Maximum allowed funding rate (e9 format). | [default to undefined]
**defaultMakerFeeE9** | **string** | Default maker fee (e9 format). | [default to undefined]
**defaultTakerFeeE9** | **string** | Default taker fee (e9 format). | [default to undefined]
**insurancePoolAddress** | **string** | Insurance pool address. | [default to undefined]
**feePoolAddress** | **string** | Fee pool address. | [default to undefined]
**tradingStartTimeAtMillis** | **string** | The time when trading will start/have started on the market. | [default to undefined]
**mtbLongE9** | **string** | Maximum take bound for long positions (e9 format). | [default to undefined]
**mtbShortE9** | **string** | Maximum take bound for short positions (e9 format). | [default to undefined]
**delistingPriceE9** | **string** | Delisting price (e9 format). | [default to undefined]
**isolatedOnly** | **boolean** | Indicates whether the market only allows isolated margin. | [default to undefined]

## Example

```typescript
import { Market } from '@bluefin/api-client';

const instance: Market = {
    symbol,
    marketAddress,
    status,
    baseAssetSymbol,
    baseAssetName,
    baseAssetDecimals,
    stepSizeE9,
    tickSizeE9,
    minOrderQuantityE9,
    maxLimitOrderQuantityE9,
    maxMarketOrderQuantityE9,
    minOrderPriceE9,
    maxOrderPriceE9,
    maintenanceMarginRatioE9,
    initialMarginRatioE9,
    insurancePoolRatioE9,
    defaultLeverageE9,
    maxNotionalAtOpenE9,
    minTradeQuantityE9,
    maxTradeQuantityE9,
    minTradePriceE9,
    maxTradePriceE9,
    maxFundingRateE9,
    defaultMakerFeeE9,
    defaultTakerFeeE9,
    insurancePoolAddress,
    feePoolAddress,
    tradingStartTimeAtMillis,
    mtbLongE9,
    mtbShortE9,
    delistingPriceE9,
    isolatedOnly,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
