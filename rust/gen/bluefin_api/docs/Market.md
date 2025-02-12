# Market

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | Symbol of the market. | 
**status** | [**models::MarketStatus**](MarketStatus.md) |  | 
**base_asset_symbol** | **String** | Base asset symbol. | 
**base_asset_name** | **String** | Base asset name. | 
**base_asset_decimals** | **i64** | Precision of the base asset. | 
**step_size_e9** | **String** | Step size for the quantity (e9 format). | 
**tick_size_e9** | **String** | Price increment size (e9 format). | 
**min_order_quantity_e9** | **String** | Minimum order size (e9 format). | 
**max_limit_order_quantity_e9** | **String** | Maximum limit order size (e9 format). | 
**max_market_order_quantity_e9** | **String** | Maximum market order size (e9 format). | 
**min_order_price_e9** | **String** | Minimum order price (e9 format). | 
**max_order_price_e9** | **String** | Maximum order price (e9 format). | 
**maintenance_margin_ratio_e9** | **String** | Maintenance margin ratio (MMR, e9 format). | 
**initial_margin_ratio_e9** | **String** | Initial margin ratio (IMR), e9 format). | 
**insurance_pool_ratio_e9** | **String** | Insurance pool ratio (e9 format). | 
**default_leverage_e9** | **String** | Default leverage (e9 format). | 
**max_notional_at_open_e9** | **Vec<String>** | Maximum notional value at current leverage. Index 0 is max notional value for leverage set to 1x, index 1 is for leverage 2x, etc... | 
**min_trade_quantity_e9** | **String** | Minimum trade quantity allowed (e9 format). | 
**max_trade_quantity_e9** | **String** | Max trade quantity allowed (e9 format). | 
**min_trade_price_e9** | **String** | Minimum trade price allowed (e9 format). | 
**max_trade_price_e9** | **String** | Maximum trade price allowed (e9 format). | 
**max_funding_rate_e9** | **String** | Maximum allowed funding rate (e9 format). | 
**default_maker_fee_e9** | **String** | Default maker fee (e9 format). | 
**default_taker_fee_e9** | **String** | Default taker fee (e9 format). | 
**insurance_pool_address** | **String** | Insurance pool address. | 
**fee_pool_address** | **String** | Fee pool address. | 
**trading_start_time_at_utc_millis** | **String** | The time when trading will start/have started on the market. | 
**mtb_long_e9** | **String** | Maximum take bound for long positions (e9 format). | 
**mtb_short_e9** | **String** | Maximum take bound for short positions (e9 format). | 
**delisting_price_e9** | **String** | Delisting price (e9 format). | 
**isolated_only** | **bool** | Indicates whether the market only allows isolated margin. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


