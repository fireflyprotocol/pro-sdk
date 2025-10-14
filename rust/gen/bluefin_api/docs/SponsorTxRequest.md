# SponsorTxRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_bytes** | **String** | Base64 encoded transaction bytes to be sponsored.  To create txBytes: 1. Create a TransactionBlock with move calls (e.g., deposit_to_asset_bank) 2. Serialize the TransactionBlock to bytes using BCS (Binary Canonical Serialization) 3. Encode the bytes to base64 string Reference implementation: https://github.com/fireflyprotocol/library-sui/blob/1412fff4d4fe7cf6b7ec04d700e777628c57c70a/src/classes/SuiBlocks.ts#L220  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


