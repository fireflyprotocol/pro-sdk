# KoraUserBonusResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**userAddress** | **string** | User wallet address | [default to undefined]
**epochNumber** | **number** | Epoch number for the bonus | [default to undefined]
**bonusPoints** | **string** | Total bonus points earned | [default to undefined]
**bonusCcRewards** | **string** | Total bonus CC rewards earned | [default to undefined]
**criteria** | **string** | Criteria for earning the bonus | [optional] [default to undefined]

## Example

```typescript
import { KoraUserBonusResponse } from '@bluefin/api-client';

const instance: KoraUserBonusResponse = {
    userAddress,
    epochNumber,
    bonusPoints,
    bonusCcRewards,
    criteria,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
