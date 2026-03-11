# LeaderboardEntry1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rank** | **int** | Rank by lifetime points (1-based). | 
**user_address** | **str** | Wallet address (may be partially masked in response). | 
**total_points** | **str** | Lifetime Vera Points total. | 
**current_tier** | **str** |  | 

## Example

```python
from openapi_client.models.leaderboard_entry1 import LeaderboardEntry1

# TODO update the JSON string below
json = "{}"
# create an instance of LeaderboardEntry1 from a JSON string
leaderboard_entry1_instance = LeaderboardEntry1.from_json(json)
# print the JSON string representation of the object
print(LeaderboardEntry1.to_json())

# convert the object into a dict
leaderboard_entry1_dict = leaderboard_entry1_instance.to_dict()
# create an instance of LeaderboardEntry1 from a dict
leaderboard_entry1_from_dict = LeaderboardEntry1.from_dict(leaderboard_entry1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


