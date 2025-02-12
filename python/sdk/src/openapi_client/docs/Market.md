# Market


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | Symbol of the market. | 
**status** | [**MarketStatus**](MarketStatus.md) |  | 
**base_asset_symbol** | **str** | Base asset symbol. | 
**base_asset_name** | **str** | Base asset name. | 
**base_asset_decimals** | **int** | Precision of the base asset. | 
**step_size_e9** | **str** | Step size for the quantity (e9 format). | 
**tick_size_e9** | **str** | Price increment size (e9 format). | 
**min_order_quantity_e9** | **str** | Minimum order size (e9 format). | 
**max_limit_order_quantity_e9** | **str** | Maximum limit order size (e9 format). | 
**max_market_order_quantity_e9** | **str** | Maximum market order size (e9 format). | 
**min_order_price_e9** | **str** | Minimum order price (e9 format). | 
**max_order_price_e9** | **str** | Maximum order price (e9 format). | 
**maintenance_margin_ratio_e9** | **str** | Maintenance margin ratio (MMR, e9 format). | 
**initial_margin_ratio_e9** | **str** | Initial margin ratio (IMR), e9 format). | 
**insurance_pool_ratio_e9** | **str** | Insurance pool ratio (e9 format). | 
**default_leverage_e9** | **str** | Default leverage (e9 format). | 
**max_notional_at_open_e9** | **List[str]** | Maximum notional value at current leverage. Index 0 is max notional value for leverage set to 1x, index 1 is for leverage 2x, etc... | 
**min_trade_quantity_e9** | **str** | Minimum trade quantity allowed (e9 format). | 
**max_trade_quantity_e9** | **str** | Max trade quantity allowed (e9 format). | 
**min_trade_price_e9** | **str** | Minimum trade price allowed (e9 format). | 
**max_trade_price_e9** | **str** | Maximum trade price allowed (e9 format). | 
**max_funding_rate_e9** | **str** | Maximum allowed funding rate (e9 format). | 
**default_maker_fee_e9** | **str** | Default maker fee (e9 format). | 
**default_taker_fee_e9** | **str** | Default taker fee (e9 format). | 
**insurance_pool_address** | **str** | Insurance pool address. | 
**fee_pool_address** | **str** | Fee pool address. | 
**trading_start_time_at_utc_millis** | **str** | The time when trading will start/have started on the market. | 
**mtb_long_e9** | **str** | Maximum take bound for long positions (e9 format). | 
**mtb_short_e9** | **str** | Maximum take bound for short positions (e9 format). | 
**delisting_price_e9** | **str** | Delisting price (e9 format). | 
**isolated_only** | **bool** | Indicates whether the market only allows isolated margin. | 

## Example

```python
from openapi_client.models.market import Market

# TODO update the JSON string below
json = "{}"
# create an instance of Market from a JSON string
market_instance = Market.from_json(json)
# print the JSON string representation of the object
print(Market.to_json())

# convert the object into a dict
market_dict = market_instance.to_dict()
# create an instance of Market from a dict
market_from_dict = Market.from_dict(market_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


