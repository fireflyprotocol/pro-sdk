# LeaderboardResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**List[LeaderboardEntry]**](LeaderboardEntry.md) |  | 

## Example

```python
from openapi_client.models.leaderboard_response import LeaderboardResponse

# TODO update the JSON string below
json = "{}"
# create an instance of LeaderboardResponse from a JSON string
leaderboard_response_instance = LeaderboardResponse.from_json(json)
# print the JSON string representation of the object
print(LeaderboardResponse.to_json())

# convert the object into a dict
leaderboard_response_dict = leaderboard_response_instance.to_dict()
# create an instance of LeaderboardResponse from a dict
leaderboard_response_from_dict = LeaderboardResponse.from_dict(leaderboard_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


