# KoraCampaignBonus


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bonusType** | **string** | Type of bonus (e.g., VOLUME_BONUS, REFERRAL_BONUS) | [optional] [default to undefined]
**bonusCcReward** | **string** | CC reward amount for this bonus | [optional] [default to undefined]
**bonusPoints** | **string** | Points earned for this bonus | [optional] [default to undefined]
**bonusCriteria** | **string** | Criteria that was met for earning this bonus | [optional] [default to undefined]

## Example

```typescript
import { KoraCampaignBonus } from '@bluefin/api-client';

const instance: KoraCampaignBonus = {
    bonusType,
    bonusCcReward,
    bonusPoints,
    bonusCriteria,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
