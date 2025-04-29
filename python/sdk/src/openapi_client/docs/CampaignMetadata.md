# CampaignMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **str** |  | 
**campaign_name** | **str** | Name of the campaign. | 
**parent_campaign_name** | **str** | Name of the parent campaign. | 
**start_date** | **int** | Time in milliseconds for campaign start date. | 
**end_date** | **int** | Time in milliseconds for campaign end date. | 

## Example

```python
from openapi_client.models.campaign_metadata import CampaignMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of CampaignMetadata from a JSON string
campaign_metadata_instance = CampaignMetadata.from_json(json)
# print the JSON string representation of the object
print(CampaignMetadata.to_json())

# convert the object into a dict
campaign_metadata_dict = campaign_metadata_instance.to_dict()
# create an instance of CampaignMetadata from a dict
campaign_metadata_from_dict = CampaignMetadata.from_dict(campaign_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


