# TickerResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | Market symbol. | 
**last_quantity_e9** | **str** | Last trade quantity (e9 format). | 
**last_time_at_millis** | **int** | Last trade time in milliseconds. | 
**last_price_e9** | **str** | Last trade price (e9 format). | 
**last_funding_rate_e9** | **str** | Funding rate value (e9 format). | 
**next_funding_time_at_millis** | **int** | Time in milliseconds of next funding rate update. | 
**avg_funding_rate8hr_e9** | **str** | 8 hr average funding rate (e9 format). | 
**oracle_price_e9** | **str** | Oracle price of the asset (e9 format). | 
**oracle_price_direction** | **int** | Direction of oracle price computed by comparing current oracle price to last oracle price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 positive trend (current &gt; last). | 
**mark_price_e9** | **str** | Mark price on the exchange (e9 format). | 
**mark_price_direction** | **int** | Direction of mark price computed by comparing current mark price to last mark price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 positive trend (current &gt; last). | 
**market_price_e9** | **str** | Simple average of bestBid and bestAsk. lastPrice if either is not present (e9 format). | 
**market_price_direction** | **int** | Direction of market price computed by comparing current market price to last market price. 0 &#x3D; no change, -1 &#x3D; negative trend (current &lt; last), 1 positive trend (current &gt; last). | 
**best_bid_price_e9** | **str** | Best bid price (e9 format). | 
**best_bid_quantity_e9** | **str** | Best bid quantity (e9 format). | 
**best_ask_price_e9** | **str** | Best ask price (e9 format). | 
**best_ask_quantity_e9** | **str** | Best ask quantity (e9 format). | 
**open_interest_e9** | **str** | Open interest value (e9 format). | 
**high_price24hr_e9** | **str** | Highest Price in the last 24hrs (e9 format). | 
**low_price24hr_e9** | **str** | Lowest Price in the last 24hrs (e9 format). | 
**volume24hr_e9** | **str** | Total market volume in last 24hrs of asset (e9 format). | 
**quote_volume24hr_e9** | **str** | Total market volume in last 24hrs in USDC (e9 format). | 
**close_price24hr_e9** | **str** | Close price 24hrs ago (e9 format). | 
**open_price24hr_e9** | **str** | Open price in the last 24hrs (e9 format). | 
**close_time24hr_at_millis** | **int** | 24 hour close time in milliseconds. | 
**open_time24hr_at_millis** | **int** | 24 hour open time in milliseconds. | 
**first_id24hr** | **int** | First trade id in 24hr. | 
**last_id24hr** | **int** | Last trade id in 24hr. | 
**count24hr** | **str** | Total number of trades in 24hr. | 
**price_change24hr_e9** | **str** | 24hr Market price change (e9 format). | 
**price_change_percent24hr_e9** | **str** | 24hr Market price change in percentage (e9 format). | 
**updated_at_millis** | **int** | Last update time in milliseconds. | 

## Example

```python
from openapi_client.models.ticker_response import TickerResponse

# TODO update the JSON string below
json = "{}"
# create an instance of TickerResponse from a JSON string
ticker_response_instance = TickerResponse.from_json(json)
# print the JSON string representation of the object
print(TickerResponse.to_json())

# convert the object into a dict
ticker_response_dict = ticker_response_instance.to_dict()
# create an instance of TickerResponse from a dict
ticker_response_from_dict = TickerResponse.from_dict(ticker_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


