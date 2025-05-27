# CandlestickUpdate

Represents a candlestick for a specific market and interval.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the market for this candlestick. | [default to undefined]
**startTime** | **number** | The start time of the candlestick in milliseconds since epoch. | [default to undefined]
**endTime** | **number** | The end time of the candlestick in milliseconds since epoch. | [default to undefined]
**interval** | **string** | The interval of the candlestick (e.g., 1m, 5m, 1h). | [default to undefined]
**openPriceE9** | **string** | The opening price of the candlestick. | [default to undefined]
**closePriceE9** | **string** | The closing price of the candlestick. | [default to undefined]
**highPriceE9** | **string** | The highest price during the candlestick interval. | [default to undefined]
**lowPriceE9** | **string** | The lowest price during the candlestick interval. | [default to undefined]
**volumeE9** | **string** | The total trading volume during the candlestick interval. | [default to undefined]
**quoteVolumeE9** | **string** | The total quote volume traded during the candlestick interval. | [default to undefined]
**numTrades** | **number** | The number of trades that occurred during the candlestick interval. | [default to undefined]

## Example

```typescript
import { CandlestickUpdate } from '@bluefin/api-client';

const instance: CandlestickUpdate = {
    symbol,
    startTime,
    endTime,
    interval,
    openPriceE9,
    closePriceE9,
    highPriceE9,
    lowPriceE9,
    volumeE9,
    quoteVolumeE9,
    numTrades,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
