# AffiliateLeaderDashboard


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | **str** | The user&#39;s wallet address | 
**name** | **str** | Name of the affiliate | [optional] 
**parent_address** | **str** | The parent affiliate&#39;s wallet address | 
**parent_name** | **str** | Name of the parent affiliate | [optional] 
**perps_rank** | **int** | Ranking in perps trading category | 
**spot_rank** | **int** | Ranking in spot trading category | 
**lending_rank** | **int** | Ranking in lending category | 
**perps_total_earnings_e9** | **str** | Total earnings from perps trading (e9 format) | 
**spot_total_earnings_e9** | **str** | Total earnings from spot trading (e9 format) | 
**lending_total_earnings_e9** | **str** | Total earnings from lending (e9 format) | 

## Example

```python
from openapi_client.models.affiliate_leader_dashboard import AffiliateLeaderDashboard

# TODO update the JSON string below
json = "{}"
# create an instance of AffiliateLeaderDashboard from a JSON string
affiliate_leader_dashboard_instance = AffiliateLeaderDashboard.from_json(json)
# print the JSON string representation of the object
print(AffiliateLeaderDashboard.to_json())

# convert the object into a dict
affiliate_leader_dashboard_dict = affiliate_leader_dashboard_instance.to_dict()
# create an instance of AffiliateLeaderDashboard from a dict
affiliate_leader_dashboard_from_dict = AffiliateLeaderDashboard.from_dict(affiliate_leader_dashboard_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


