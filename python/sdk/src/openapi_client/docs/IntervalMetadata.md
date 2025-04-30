# IntervalMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | 
**start_date** | **int** | Time in milliseconds for interval start date. | 
**end_date** | **int** | Time in milliseconds for interval end date. | 
**interval_id** | **int** | Interval Id of the interval. | 
**interval_type** | **str** | Type of the interval. | 

## Example

```python
from openapi_client.models.interval_metadata import IntervalMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of IntervalMetadata from a JSON string
interval_metadata_instance = IntervalMetadata.from_json(json)
# print the JSON string representation of the object
print(IntervalMetadata.to_json())

# convert the object into a dict
interval_metadata_dict = interval_metadata_instance.to_dict()
# create an instance of IntervalMetadata from a dict
interval_metadata_from_dict = IntervalMetadata.from_dict(interval_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


