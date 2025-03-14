# OrderbookDepthResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **String** | Market symbol. | 
**last_update_id** | **i64** | Count indicating the number of changes in orderbook state. | 
**last_updated_at_millis** | **i64** | Timestamp at which the last change in orderbook state took place, in milliseconds. | 
**response_sent_at_millis** | **i64** | The time at which the orderbook server sent the response, in milliseconds. | 
**best_bid_price_e9** | **String** | The best bid price on orderbook at the moment (e9 format). | 
**best_bid_quantity_e9** | **String** | The best bid quantity on orderbook at the moment (e9 format). | 
**best_ask_price_e9** | **String** | The best ask price on orderbook at the moment (e9 format). | 
**best_ask_quantity_e9** | **String** | The best ask quantity on orderbook at the moment (e9 format). | 
**bids_e9** | [**Vec<Vec<String>>**](Vec.md) | Bids to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format. | 
**asks_e9** | [**Vec<Vec<String>>**](Vec.md) | Asks to be filled. Index 0 is price, index 1 is quantity at price bin. Prices are in e9 format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


