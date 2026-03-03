# KoraLeaderboardEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rank** | **int** | Overall ranking position based on sortBy parameter | 
**user_address** | **str** | Wallet address of the user | 
**total_volume_e6** | **str** | Total swap volume (USD equivalent) | 
**total_fee_e6** | **str** | Total fees paid | [optional] 
**total_transactions** | **int** | Total number of qualifying swap transactions | 
**cc_rewards** | **str** | CC token rewards earned | 
**kora_points** | **str** | Kora points earned | 
**total_bonus_points** | **str** | Total bonus points earned | [optional] 
**total_bonus_cc** | **str** | Total bonus CC tokens earned | [optional] 
**volume_rank** | **int** | Ranking based purely on trading volume | [optional] 
**transaction_rank** | **int** | Ranking based purely on transaction count | [optional] 
**total_earnings_rank** | **int** | Ranking based on combined CC + Kora points value | [optional] 
**last_activity_date** | **int** | Timestamp of user&#39;s most recent swap (milliseconds) | [optional] 

## Example

```python
from openapi_client.models.kora_leaderboard_entry import KoraLeaderboardEntry

# TODO update the JSON string below
json = "{}"
# create an instance of KoraLeaderboardEntry from a JSON string
kora_leaderboard_entry_instance = KoraLeaderboardEntry.from_json(json)
# print the JSON string representation of the object
print(KoraLeaderboardEntry.to_json())

# convert the object into a dict
kora_leaderboard_entry_dict = kora_leaderboard_entry_instance.to_dict()
# create an instance of KoraLeaderboardEntry from a dict
kora_leaderboard_entry_from_dict = KoraLeaderboardEntry.from_dict(kora_leaderboard_entry_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


