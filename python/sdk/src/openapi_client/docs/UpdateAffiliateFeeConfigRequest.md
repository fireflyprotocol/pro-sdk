# UpdateAffiliateFeeConfigRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cashback** | **int** | Cashback amount to give to the referees | 

## Example

```python
from openapi_client.models.update_affiliate_fee_config_request import UpdateAffiliateFeeConfigRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateAffiliateFeeConfigRequest from a JSON string
update_affiliate_fee_config_request_instance = UpdateAffiliateFeeConfigRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateAffiliateFeeConfigRequest.to_json())

# convert the object into a dict
update_affiliate_fee_config_request_dict = update_affiliate_fee_config_request_instance.to_dict()
# create an instance of UpdateAffiliateFeeConfigRequest from a dict
update_affiliate_fee_config_request_from_dict = UpdateAffiliateFeeConfigRequest.from_dict(update_affiliate_fee_config_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


