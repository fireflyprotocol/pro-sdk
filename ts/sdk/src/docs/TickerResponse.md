# TickerResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | Market symbol. | [default to undefined]
**lastQuantityE9** | **string** | Last trade quantity (e9 format). | [default to undefined]
**lastTimeAtMillis** | **number** | Last trade time in milliseconds. | [default to undefined]
**lastPriceE9** | **string** | Last trade price (e9 format). | [default to undefined]
**lastFundingRateE9** | **string** | Funding rate value (e9 format). | [default to undefined]
**nextFundingTimeAtMillis** | **number** | Time in milliseconds of next funding rate update. | [default to undefined]
**avgFundingRate8hrE9** | **string** | 8 hr average funding rate (e9 format). | [default to undefined]
**oraclePriceE9** | **string** | Oracle price of the asset (e9 format). | [default to undefined]
**oraclePriceDirection** | **number** | Direction of oracle price computed by comparing current oracle price to last oracle price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 positive trend (current &gt; last). | [default to undefined]
**markPriceE9** | **string** | Mark price on the exchange (e9 format). | [default to undefined]
**markPriceDirection** | **number** | Direction of mark price computed by comparing current mark price to last mark price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 positive trend (current &gt; last). | [default to undefined]
**marketPriceE9** | **string** | Simple average of bestBid and bestAsk. lastPrice if either is not present (e9 format). | [default to undefined]
**marketPriceDirection** | **number** | Direction of market price computed by comparing current market price to last market price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 positive trend (current &gt; last). | [default to undefined]
**bestBidPriceE9** | **string** | Best bid price (e9 format). | [default to undefined]
**bestBidQuantityE9** | **string** | Best bid quantity (e9 format). | [default to undefined]
**bestAskPriceE9** | **string** | Best ask price (e9 format). | [default to undefined]
**bestAskQuantityE9** | **string** | Best ask quantity (e9 format). | [default to undefined]
**openInterestE9** | **string** | Open interest value (e9 format). | [default to undefined]
**highPrice24hrE9** | **string** | Highest Price in the last 24hrs (e9 format). | [default to undefined]
**lowPrice24hrE9** | **string** | Lowest Price in the last 24hrs (e9 format). | [default to undefined]
**volume24hrE9** | **string** | Total market volume in last 24hrs of asset (e9 format). | [default to undefined]
**quoteVolume24hrE9** | **string** | Total market volume in last 24hrs in USDC (e9 format). | [default to undefined]
**closePrice24hrE9** | **string** | Close price 24hrs ago (e9 format). | [default to undefined]
**openPrice24hrE9** | **string** | Open price in the last 24hrs (e9 format). | [default to undefined]
**closeTime24hrAtMillis** | **number** | 24 hour close time in milliseconds. | [default to undefined]
**openTime24hrAtMillis** | **number** | 24 hour open time in milliseconds. | [default to undefined]
**firstId24hr** | **number** | First trade id in 24hr. | [default to undefined]
**lastId24hr** | **number** | Last trade id in 24hr. | [default to undefined]
**count24hr** | **string** | Total number of trades in 24hr. | [default to undefined]
**priceChange24hrE9** | **string** | 24hr Market price change (e9 format). | [default to undefined]
**priceChangePercent24hrE9** | **string** | 24hr Market price change in percentage (e9 format). | [default to undefined]
**updatedAtMillis** | **number** | Last update time in milliseconds. | [default to undefined]
**estimatedFundingRateE9** | **string** | Live estimated funding rate based on current hour\&#39;s average market and oracle prices (e9 format). | [default to undefined]

## Example

```typescript
import { TickerResponse } from '@bluefin/api-client';

const instance: TickerResponse = {
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
    estimatedFundingRateE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
