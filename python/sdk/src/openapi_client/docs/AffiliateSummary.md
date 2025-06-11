# AffiliateSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | The user&#39;s wallet address | 
**total_referred_users** | **int** | Total number of referees | 
**total_earnings_e9** | **str** | Total earnings in e9 format | 
**perps_ranking** | **int** | Ranking in perps trading category | 
**spot_ranking** | **int** | Ranking in spot trading category | 
**lend_ranking** | **int** | Ranking in lending category | 

## Example

```python
from openapi_client.models.affiliate_summary import AffiliateSummary

# TODO update the JSON string below
json = "{}"
# create an instance of AffiliateSummary from a JSON string
affiliate_summary_instance = AffiliateSummary.from_json(json)
# print the JSON string representation of the object
print(AffiliateSummary.to_json())

# convert the object into a dict
affiliate_summary_dict = affiliate_summary_instance.to_dict()
# create an instance of AffiliateSummary from a dict
affiliate_summary_from_dict = AffiliateSummary.from_dict(affiliate_summary_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


