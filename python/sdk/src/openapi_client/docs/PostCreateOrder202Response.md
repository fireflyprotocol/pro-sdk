# PostCreateOrder202Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_hash** | **str** | The unique identifier of this order, to be used as a lookup key | 

## Example

```python
from openapi_client.models.post_create_order202_response import PostCreateOrder202Response

# TODO update the JSON string below
json = "{}"
# create an instance of PostCreateOrder202Response from a JSON string
post_create_order202_response_instance = PostCreateOrder202Response.from_json(json)
# print the JSON string representation of the object
print(PostCreateOrder202Response.to_json())

# convert the object into a dict
post_create_order202_response_dict = post_create_order202_response_instance.to_dict()
# create an instance of PostCreateOrder202Response from a dict
post_create_order202_response_from_dict = PostCreateOrder202Response.from_dict(post_create_order202_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


