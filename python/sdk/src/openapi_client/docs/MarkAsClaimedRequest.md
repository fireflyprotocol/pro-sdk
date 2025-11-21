# MarkAsClaimedRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval_number** | **int** | The interval number | 
**campaign_name** | **str** | The campaign name | 
**txn_digest** | **str** | The transaction digest of the claim | 
**reward_types** | **List[str]** | The reward types to mark as claimed | 

## Example

```python
from openapi_client.models.mark_as_claimed_request import MarkAsClaimedRequest

# TODO update the JSON string below
json = "{}"
# create an instance of MarkAsClaimedRequest from a JSON string
mark_as_claimed_request_instance = MarkAsClaimedRequest.from_json(json)
# print the JSON string representation of the object
print(MarkAsClaimedRequest.to_json())

# convert the object into a dict
mark_as_claimed_request_dict = mark_as_claimed_request_instance.to_dict()
# create an instance of MarkAsClaimedRequest from a dict
mark_as_claimed_request_from_dict = MarkAsClaimedRequest.from_dict(mark_as_claimed_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


