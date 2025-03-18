# OrderbookDiffDepthUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at_millis** | **i64** | The timestamp of the orderbook update. | 
**symbol** | **String** | The symbol of the market for the orderbook update. | 
**bids_e9** | [**Vec<Vec<String>>**](Vec.md) |  | 
**asks_e9** | [**Vec<Vec<String>>**](Vec.md) |  | 
**first_update_id** | **i64** | The ID of the first update in this batch. | 
**last_update_id** | **i64** | The ID of the last update in this batch. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


