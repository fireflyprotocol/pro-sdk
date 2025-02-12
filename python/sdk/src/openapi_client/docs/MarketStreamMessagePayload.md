# MarketStreamMessagePayload

The payload of the message, which varies based on the event type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recent_trades** | [**List[RecentTradesUpdate]**](RecentTradesUpdate.md) |  | 
**symbol** | **str** | The symbol of the market for the partial depth update. | 
**last_quantity_e9** | **str** | Last trade quantity (e9 format). | 
**last_time_at_utc_millis** | **int** | Last trade time in milliseconds. | 
**last_price_e9** | **str** | Last trade price (e9 format). | 
**last_funding_rate_e9** | **str** | Funding rate value (e9 format). | 
**next_funding_time_at_utc_millis** | **int** | Time in milliseconds of next funding rate update. | 
**avg_funding_rate8hr_e9** | **str** | 8 hr average funding rate (e9 format). | 
**oracle_price_e9** | **str** | Oracle price of the asset (e9 format). | 
**oracle_price_direction** | **int** | Direction of oracle price computed by comparing current oracle price to last oracle price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 &#x3D; positive trend (current &gt; last). | 
**mark_price_e9** | **str** | Mark price on the exchange (e9 format). | 
**mark_price_direction** | **int** | Direction of mark price computed by comparing current mark price to last mark price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 &#x3D; positive trend (current &gt; last). | 
**market_price_e9** | **str** | Simple average of bestBid and bestAsk, or lastPrice if either is not present (e9 format). | 
**market_price_direction** | **int** | Direction of market price computed by comparing current market price to last market price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 &#x3D; positive trend (current &gt; last). | 
**best_bid_price_e9** | **str** | Best bid price (e9 format). | 
**best_bid_quantity_e9** | **str** | Best bid quantity (e9 format). | 
**best_ask_price_e9** | **str** | Best ask price (e9 format). | 
**best_ask_quantity_e9** | **str** | Best ask quantity (e9 format). | 
**open_interest_e9** | **str** | Open interest value (e9 format). | 
**high_price24hr_e9** | **str** | Highest Price in the last 24 hours (e9 format). | 
**low_price24hr_e9** | **str** | Lowest Price in the last 24 hours (e9 format). | 
**volume24hr_e9** | **str** | Total market volume in last 24 hours of asset (e9 format). | 
**quote_volume24hr_e9** | **str** | Total market volume in last 24 hours in USDC (e9 format). | 
**close_price24hr_e9** | **str** | Close price 24 hours ago (e9 format). | 
**open_price24hr_e9** | **str** | Open price in the last 24 hours (e9 format). | 
**close_time24hr_at_utc_millis** | **int** | 24 hour close timestamp in milliseconds. | 
**open_time24hr_at_utc_millis** | **int** | 24 hour open timetamp in milliseconds. | 
**first_id24hr** | **int** | First trade ID in the last 24 hours. | 
**last_id24hr** | **int** | Last trade ID in the last 24 hours. | 
**count24hr** | **str** | Total number of trades in the last 24 hours. | 
**price_change24hr_e9** | **str** | 24 hour Market price change (e9 format). | 
**price_change_percent24hr_e9** | **str** | 24 hour Market price change as a percentage (e9 format). | 
**last_updated_at_utc_millis** | **int** | Last update timestamp in milliseconds. | 
**ticker_all** | [**List[TickerUpdate]**](TickerUpdate.md) | Array of detailed market ticker information for all markets. | 
**price_e9** | **str** | The price in scientific notation with 9 decimal places of precision. | 
**source** | **str** |  | 
**updated_at_utc_millis** | **int** | The timestamp of the partial depth update. | 
**start_time** | **int** | The start time of the candlestick in milliseconds since epoch. | 
**end_time** | **int** | The end time of the candlestick in milliseconds since epoch. | 
**interval** | **str** | The interval of the candlestick (e.g., 1m, 5m, 1h). | 
**open_price_e9** | **str** | The opening price of the candlestick. | 
**close_price_e9** | **str** | The closing price of the candlestick. | 
**high_price_e9** | **str** | The highest price during the candlestick interval. | 
**low_price_e9** | **str** | The lowest price during the candlestick interval. | 
**volume_e9** | **str** | The total trading volume during the candlestick interval. | 
**quote_volume_e9** | **str** | The total quote volume traded during the candlestick interval. | 
**num_trades** | **int** | The number of trades that occurred during the candlestick interval. | 
**bids_e9** | **List[List[str]]** |  | 
**asks_e9** | **List[List[str]]** |  | 
**first_update_id** | **int** | The ID of the first update in this batch. | 
**last_update_id** | **int** | The ID of the last update in this batch. | 
**orderbook_update_id** | **int** | The unique identifier for the orderbook update. | 
**depth_level** | **str** | The depth level of the orderbook snapshot. | 

## Example

```python
from openapi_client.models.market_stream_message_payload import MarketStreamMessagePayload

# TODO update the JSON string below
json = "{}"
# create an instance of MarketStreamMessagePayload from a JSON string
market_stream_message_payload_instance = MarketStreamMessagePayload.from_json(json)
# print the JSON string representation of the object
print(MarketStreamMessagePayload.to_json())

# convert the object into a dict
market_stream_message_payload_dict = market_stream_message_payload_instance.to_dict()
# create an instance of MarketStreamMessagePayload from a dict
market_stream_message_payload_from_dict = MarketStreamMessagePayload.from_dict(market_stream_message_payload_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


