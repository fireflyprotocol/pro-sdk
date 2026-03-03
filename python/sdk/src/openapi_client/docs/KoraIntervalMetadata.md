# KoraIntervalMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | 
**start_date** | **int** | Time in milliseconds for interval start date | 
**end_date** | **int** | Time in milliseconds for interval end date | 
**interval_id** | **int** | Interval ID | 
**interval_type** | **str** | Type of the interval | [optional] 
**protocol** | **str** | Protocol for the interval | [optional] 

## Example

```python
from openapi_client.models.kora_interval_metadata import KoraIntervalMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of KoraIntervalMetadata from a JSON string
kora_interval_metadata_instance = KoraIntervalMetadata.from_json(json)
# print the JSON string representation of the object
print(KoraIntervalMetadata.to_json())

# convert the object into a dict
kora_interval_metadata_dict = kora_interval_metadata_instance.to_dict()
# create an instance of KoraIntervalMetadata from a dict
kora_interval_metadata_from_dict = KoraIntervalMetadata.from_dict(kora_interval_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


