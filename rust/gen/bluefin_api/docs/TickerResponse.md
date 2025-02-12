# TickerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | Market symbol. | 
**last_quantity_e9** | **String** | Last trade quantity (e9 format). | 
**last_time_at_utc_millis** | **i64** | Last trade time in milliseconds. | 
**last_price_e9** | **String** | Last trade price (e9 format). | 
**last_funding_rate_e9** | **String** | Funding rate value (e9 format). | 
**next_funding_time_at_utc_millis** | **i64** | Time in milliseconds of next funding rate update. | 
**avg_funding_rate8hr_e9** | **String** | 8 hr average funding rate (e9 format). | 
**oracle_price_e9** | **String** | Oracle price of the asset (e9 format). | 
**oracle_price_direction** | **i64** | Direction of oracle price computed by comparing current oracle price to last oracle price. 0 = no change, -1 = negative trend (current < last), 1 positive trend (current > last). | 
**mark_price_e9** | **String** | Mark price on the exchange (e9 format). | 
**mark_price_direction** | **i64** | Direction of mark price computed by comparing current mark price to last mark price. 0 = no change, -1 = negative trend (current < last), 1 positive trend (current > last). | 
**market_price_e9** | **String** | Simple average of bestBid and bestAsk. lastPrice if either is not present (e9 format). | 
**market_price_direction** | **i32** | Direction of market price computed by comparing current market price to last market price. 0 = no change, -1 = negative trend (current < last), 1 positive trend (current > last). | 
**best_bid_price_e9** | **String** | Best bid price (e9 format). | 
**best_bid_quantity_e9** | **String** | Best bid quantity (e9 format). | 
**best_ask_price_e9** | **String** | Best ask price (e9 format). | 
**best_ask_quantity_e9** | **String** | Best ask quantity (e9 format). | 
**open_interest_e9** | **String** | Open interest value (e9 format). | 
**high_price24hr_e9** | **String** | Highest Price in the last 24hrs (e9 format). | 
**low_price24hr_e9** | **String** | Lowest Price in the last 24hrs (e9 format). | 
**volume24hr_e9** | **String** | Total market volume in last 24hrs of asset (e9 format). | 
**quote_volume24hr_e9** | **String** | Total market volume in last 24hrs in USDC (e9 format). | 
**close_price24hr_e9** | **String** | Close price 24hrs ago (e9 format). | 
**open_price24hr_e9** | **String** | Open price in the last 24hrs (e9 format). | 
**close_time24hr_at_utc_millis** | **i64** | 24 hour close time in milliseconds. | 
**open_time24hr_at_utc_millis** | **i64** | 24 hour open time in milliseconds. | 
**first_id24hr** | **i64** | First trade id in 24hr. | 
**last_id24hr** | **i64** | Last trade id in 24hr. | 
**count24hr** | **String** | Total number of trades in 24hr. | 
**price_change24hr_e9** | **String** | 24hr Market price change (e9 format). | 
**price_change_percent24hr_e9** | **String** | 24hr Market price change in percentage (e9 format). | 
**last_updated_at_utc_millis** | **i64** | Last update time in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


