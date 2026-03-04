# KoraEpochMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | 
**campaign_name** | **str** | Name of the campaign | 
**epoch_id** | **str** | Epoch ID | 
**epoch_number** | **int** | Epoch number | 
**start_date** | **int** | Time in milliseconds for epoch start date | 
**end_date** | **int** | Time in milliseconds for epoch end date | 

## Example

```python
from openapi_client.models.kora_epoch_metadata import KoraEpochMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of KoraEpochMetadata from a JSON string
kora_epoch_metadata_instance = KoraEpochMetadata.from_json(json)
# print the JSON string representation of the object
print(KoraEpochMetadata.to_json())

# convert the object into a dict
kora_epoch_metadata_dict = kora_epoch_metadata_instance.to_dict()
# create an instance of KoraEpochMetadata from a dict
kora_epoch_metadata_from_dict = KoraEpochMetadata.from_dict(kora_epoch_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


