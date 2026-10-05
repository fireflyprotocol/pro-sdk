# GetSpotRewardClaims200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**List[SpotRewardClaimCard]**](SpotRewardClaimCard.md) |  | 
**next_cursor** | **int** | claimTimestamp to pass as the next cursor; -1 when there are no rows. | 
**is_more_data_available** | **bool** |  | 

## Example

```python
from openapi_client.models.get_spot_reward_claims200_response import GetSpotRewardClaims200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetSpotRewardClaims200Response from a JSON string
get_spot_reward_claims200_response_instance = GetSpotRewardClaims200Response.from_json(json)
# print the JSON string representation of the object
print(GetSpotRewardClaims200Response.to_json())

# convert the object into a dict
get_spot_reward_claims200_response_dict = get_spot_reward_claims200_response_instance.to_dict()
# create an instance of GetSpotRewardClaims200Response from a dict
get_spot_reward_claims200_response_from_dict = GetSpotRewardClaims200Response.from_dict(get_spot_reward_claims200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


