# MarketStreamMessagePayload

The payload of the message, which varies based on the event type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trades** | [**Array&lt;Trade&gt;**](Trade.md) |  | [default to undefined]
**symbol** | **string** | The symbol of the market for the partial depth update. | [default to undefined]
**lastQuantityE9** | **string** | Last trade quantity (e9 format). | [default to undefined]
**lastTimeAtMillis** | **number** | Last trade time in milliseconds. | [default to undefined]
**lastPriceE9** | **string** | Last trade price (e9 format). | [default to undefined]
**lastFundingRateE9** | **string** | Funding rate value (e9 format). | [default to undefined]
**nextFundingTimeAtMillis** | **number** | Time in milliseconds of next funding rate update. | [default to undefined]
**avgFundingRate8hrE9** | **string** | 8 hr average funding rate (e9 format). | [default to undefined]
**oraclePriceE9** | **string** | Oracle price of the asset (e9 format). | [default to undefined]
**oraclePriceDirection** | **number** | Direction of oracle price computed by comparing current oracle price to last oracle price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 &#x3D; positive trend (current &gt; last). | [default to undefined]
**markPriceE9** | **string** | Mark price on the exchange (e9 format). | [default to undefined]
**markPriceDirection** | **number** | Direction of mark price computed by comparing current mark price to last mark price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 &#x3D; positive trend (current &gt; last). | [default to undefined]
**marketPriceE9** | **string** | Simple average of bestBid and bestAsk, or lastPrice if either is not present (e9 format). | [default to undefined]
**marketPriceDirection** | **number** | Direction of market price computed by comparing current market price to last market price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 &#x3D; positive trend (current &gt; last). | [default to undefined]
**bestBidPriceE9** | **string** | Best bid price (e9 format). | [default to undefined]
**bestBidQuantityE9** | **string** | Best bid quantity (e9 format). | [default to undefined]
**bestAskPriceE9** | **string** | Best ask price (e9 format). | [default to undefined]
**bestAskQuantityE9** | **string** | Best ask quantity (e9 format). | [default to undefined]
**openInterestE9** | **string** | Open interest value (e9 format). | [default to undefined]
**highPrice24hrE9** | **string** | Highest Price in the last 24 hours (e9 format). | [default to undefined]
**lowPrice24hrE9** | **string** | Lowest Price in the last 24 hours (e9 format). | [default to undefined]
**volume24hrE9** | **string** | Total market volume in last 24 hours of asset (e9 format). | [default to undefined]
**quoteVolume24hrE9** | **string** | Total market volume in last 24 hours in USDC (e9 format). | [default to undefined]
**closePrice24hrE9** | **string** | Close price 24 hours ago (e9 format). | [default to undefined]
**openPrice24hrE9** | **string** | Open price in the last 24 hours (e9 format). | [default to undefined]
**closeTime24hrAtMillis** | **number** | 24 hour close timestamp in milliseconds. | [default to undefined]
**openTime24hrAtMillis** | **number** | 24 hour open timetamp in milliseconds. | [default to undefined]
**firstId24hr** | **number** | First trade ID in the last 24 hours. | [default to undefined]
**lastId24hr** | **number** | Last trade ID in the last 24 hours. | [default to undefined]
**count24hr** | **string** | Total number of trades in the last 24 hours. | [default to undefined]
**priceChange24hrE9** | **string** | 24 hour Market price change (e9 format). | [default to undefined]
**priceChangePercent24hrE9** | **string** | 24 hour Market price change as a percentage (e9 format). | [default to undefined]
**updatedAtMillis** | **number** | The timestamp of the partial depth update. | [default to undefined]
**tickerAll** | [**Array&lt;TickerUpdate&gt;**](TickerUpdate.md) | Array of detailed market ticker information for all markets. | [default to undefined]
**priceE9** | **string** | The price in scientific notation with 9 decimal places of precision. | [default to undefined]
**source** | **string** |  | [default to undefined]
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
**bidsE9** | **Array&lt;Array&lt;string&gt;&gt;** |  | [default to undefined]
**asksE9** | **Array&lt;Array&lt;string&gt;&gt;** |  | [default to undefined]
**firstUpdateId** | **number** | The ID of the first update in this batch. | [default to undefined]
**lastUpdateId** | **number** | The ID of the last update in this batch. | [default to undefined]
**orderbookUpdateId** | **number** | The unique identifier for the orderbook update. | [default to undefined]
**depthLevel** | **string** | The depth level of the orderbook snapshot. | [default to undefined]

## Example

```typescript
import { MarketStreamMessagePayload } from '@bluefin/api-client';

const instance: MarketStreamMessagePayload = {
    trades,
    symbol,
    lastQuantityE9,
    lastTimeAtMillis,
    lastPriceE9,
    lastFundingRateE9,
    nextFundingTimeAtMillis,
    avgFundingRate8hrE9,
    oraclePriceE9,
    oraclePriceDirection,
    markPriceE9,
    markPriceDirection,
    marketPriceE9,
    marketPriceDirection,
    bestBidPriceE9,
    bestBidQuantityE9,
    bestAskPriceE9,
    bestAskQuantityE9,
    openInterestE9,
    highPrice24hrE9,
    lowPrice24hrE9,
    volume24hrE9,
    quoteVolume24hrE9,
    closePrice24hrE9,
    openPrice24hrE9,
    closeTime24hrAtMillis,
    openTime24hrAtMillis,
    firstId24hr,
    lastId24hr,
    count24hr,
    priceChange24hrE9,
    priceChangePercent24hrE9,
    updatedAtMillis,
    tickerAll,
    priceE9,
    source,
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
    bidsE9,
    asksE9,
    firstUpdateId,
    lastUpdateId,
    orderbookUpdateId,
    depthLevel,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
