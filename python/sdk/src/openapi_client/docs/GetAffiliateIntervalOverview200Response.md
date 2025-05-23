# GetAffiliateIntervalOverview200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**List[AffiliateIntervalOverview]**](AffiliateIntervalOverview.md) |  | [optional] 
**total** | **int** | Total number of records | [optional] 
**limit** | **int** | The page size for pagination | [optional] 
**page** | **int** | The page number for pagination | [optional] 

## Example

```python
from openapi_client.models.get_affiliate_interval_overview200_response import GetAffiliateIntervalOverview200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetAffiliateIntervalOverview200Response from a JSON string
get_affiliate_interval_overview200_response_instance = GetAffiliateIntervalOverview200Response.from_json(json)
# print the JSON string representation of the object
print(GetAffiliateIntervalOverview200Response.to_json())

# convert the object into a dict
get_affiliate_interval_overview200_response_dict = get_affiliate_interval_overview200_response_instance.to_dict()
# create an instance of GetAffiliateIntervalOverview200Response from a dict
get_affiliate_interval_overview200_response_from_dict = GetAffiliateIntervalOverview200Response.from_dict(get_affiliate_interval_overview200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


