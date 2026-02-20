# LeaderboardEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rank** | **number** | Rank of the trader in the leaderboard. | [default to undefined]
**accountAddress** | **string** | The address of the account. | [default to undefined]
**accountValueE9** | **string** | Total account value of the trader in e9 format. | [default to undefined]
**pnlE9** | **string** | Total profit and loss of the trader in e9 format. | [default to undefined]
**volumeE9** | **string** | Total trading volume of the trader in e9 format. | [default to undefined]

## Example

```typescript
import { LeaderboardEntry } from '@bluefin/api-client';

const instance: LeaderboardEntry = {
    rank,
    accountAddress,
    accountValueE9,
    pnlE9,
    volumeE9,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
