# MarketPriceUpdate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the market. | [default to undefined]
**priceE9** | **string** | The price in scientific notation with 9 decimal places of precision. | [default to undefined]
**source** | **string** |  | [default to undefined]
**updatedAtMillis** | **number** | The timestamp of the price update. | [default to undefined]

## Example

```typescript
import { MarketPriceUpdate } from '@bluefin/api-client';

const instance: MarketPriceUpdate = {
    symbol,
    priceE9,
    source,
    updatedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
