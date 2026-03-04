# KoraLeaderboardResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**List[KoraLeaderboardEntry]**](KoraLeaderboardEntry.md) |  | 
**total** | **int** | Total number of records | 
**limit** | **int** | Page size for pagination | 
**page** | **int** | Current page number | 

## Example

```python
from openapi_client.models.kora_leaderboard_response import KoraLeaderboardResponse

# TODO update the JSON string below
json = "{}"
# create an instance of KoraLeaderboardResponse from a JSON string
kora_leaderboard_response_instance = KoraLeaderboardResponse.from_json(json)
# print the JSON string representation of the object
print(KoraLeaderboardResponse.to_json())

# convert the object into a dict
kora_leaderboard_response_dict = kora_leaderboard_response_instance.to_dict()
# create an instance of KoraLeaderboardResponse from a dict
kora_leaderboard_response_from_dict = KoraLeaderboardResponse.from_dict(kora_leaderboard_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


