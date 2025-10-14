# SponsorTxRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_bytes** | **str** | Base64 encoded transaction bytes to be sponsored.  To create txBytes: 1. Create a TransactionBlock with move calls (e.g., deposit_to_asset_bank) 2. Serialize the TransactionBlock to bytes using BCS (Binary Canonical Serialization) 3. Encode the bytes to base64 string Reference implementation: https://github.com/fireflyprotocol/library-sui/blob/1412fff4d4fe7cf6b7ec04d700e777628c57c70a/src/classes/SuiBlocks.ts#L220  | 

## Example

```python
from openapi_client.models.sponsor_tx_request import SponsorTxRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SponsorTxRequest from a JSON string
sponsor_tx_request_instance = SponsorTxRequest.from_json(json)
# print the JSON string representation of the object
print(SponsorTxRequest.to_json())

# convert the object into a dict
sponsor_tx_request_dict = sponsor_tx_request_instance.to_dict()
# create an instance of SponsorTxRequest from a dict
sponsor_tx_request_from_dict = SponsorTxRequest.from_dict(sponsor_tx_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


