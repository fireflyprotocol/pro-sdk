# LeaderboardResponse1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entries** | [**List[LeaderboardEntry1]**](LeaderboardEntry1.md) |  | 
**total** | **int** | Total number of users (for pagination). | [optional] 

## Example

```python
from openapi_client.models.leaderboard_response1 import LeaderboardResponse1

# TODO update the JSON string below
json = "{}"
# create an instance of LeaderboardResponse1 from a JSON string
leaderboard_response1_instance = LeaderboardResponse1.from_json(json)
# print the JSON string representation of the object
print(LeaderboardResponse1.to_json())

# convert the object into a dict
leaderboard_response1_dict = leaderboard_response1_instance.to_dict()
# create an instance of LeaderboardResponse1 from a dict
leaderboard_response1_from_dict = LeaderboardResponse1.from_dict(leaderboard_response1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


