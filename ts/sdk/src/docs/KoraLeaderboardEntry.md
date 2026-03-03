# KoraLeaderboardEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rank** | **number** | Overall ranking position based on sortBy parameter | [default to undefined]
**userAddress** | **string** | Wallet address of the user | [default to undefined]
**totalVolumeE6** | **string** | Total swap volume (USD equivalent) | [default to undefined]
**totalFeeE6** | **string** | Total fees paid | [optional] [default to undefined]
**totalTransactions** | **number** | Total number of qualifying swap transactions | [default to undefined]
**ccRewards** | **string** | CC token rewards earned | [default to undefined]
**koraPoints** | **string** | Kora points earned | [default to undefined]
**totalBonusPoints** | **string** | Total bonus points earned | [optional] [default to undefined]
**totalBonusCC** | **string** | Total bonus CC tokens earned | [optional] [default to undefined]
**volumeRank** | **number** | Ranking based purely on trading volume | [optional] [default to undefined]
**transactionRank** | **number** | Ranking based purely on transaction count | [optional] [default to undefined]
**totalEarningsRank** | **number** | Ranking based on combined CC + Kora points value | [optional] [default to undefined]
**lastActivityDate** | **number** | Timestamp of user\&#39;s most recent swap (milliseconds) | [optional] [default to undefined]

## Example

```typescript
import { KoraLeaderboardEntry } from '@bluefin/api-client';

const instance: KoraLeaderboardEntry = {
    rank,
    userAddress,
    totalVolumeE6,
    totalFeeE6,
    totalTransactions,
    ccRewards,
    koraPoints,
    totalBonusPoints,
    totalBonusCC,
    volumeRank,
    transactionRank,
    totalEarningsRank,
    lastActivityDate,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
