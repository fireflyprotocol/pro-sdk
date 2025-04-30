# EpochMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | 
**campaign_name** | **str** | Name of the campaign. | 
**epoch_id** | **int** | Epoch Id of the epoch. | 
**epoch_number** | **int** | Epoch number for the queried epoch. | 
**start_date** | **int** | Time in milliseconds for campaign start date. | 
**end_date** | **int** | Time in milliseconds for campaign end date. | 

## Example

```python
from openapi_client.models.epoch_metadata import EpochMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of EpochMetadata from a JSON string
epoch_metadata_instance = EpochMetadata.from_json(json)
# print the JSON string representation of the object
print(EpochMetadata.to_json())

# convert the object into a dict
epoch_metadata_dict = epoch_metadata_instance.to_dict()
# create an instance of EpochMetadata from a dict
epoch_metadata_from_dict = EpochMetadata.from_dict(epoch_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


