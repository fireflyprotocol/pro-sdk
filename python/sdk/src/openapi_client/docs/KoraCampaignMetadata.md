# KoraCampaignMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | 
**campaign_name** | **str** | Name of the campaign | 
**parent_campaign_name** | **str** | Name of the parent campaign | [optional] 
**start_date** | **int** | Time in milliseconds for campaign start date | 
**end_date** | **int** | Time in milliseconds for campaign end date | 

## Example

```python
from openapi_client.models.kora_campaign_metadata import KoraCampaignMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of KoraCampaignMetadata from a JSON string
kora_campaign_metadata_instance = KoraCampaignMetadata.from_json(json)
# print the JSON string representation of the object
print(KoraCampaignMetadata.to_json())

# convert the object into a dict
kora_campaign_metadata_dict = kora_campaign_metadata_instance.to_dict()
# create an instance of KoraCampaignMetadata from a dict
kora_campaign_metadata_from_dict = KoraCampaignMetadata.from_dict(kora_campaign_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


