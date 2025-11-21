# MarkAsClaimedResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **str** | Response message indicating if the claim was marked as claimed successfully | 
**status** | **str** | Status of the claim | 

## Example

```python
from openapi_client.models.mark_as_claimed_response import MarkAsClaimedResponse

# TODO update the JSON string below
json = "{}"
# create an instance of MarkAsClaimedResponse from a JSON string
mark_as_claimed_response_instance = MarkAsClaimedResponse.from_json(json)
# print the JSON string representation of the object
print(MarkAsClaimedResponse.to_json())

# convert the object into a dict
mark_as_claimed_response_dict = mark_as_claimed_response_instance.to_dict()
# create an instance of MarkAsClaimedResponse from a dict
mark_as_claimed_response_from_dict = MarkAsClaimedResponse.from_dict(mark_as_claimed_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


