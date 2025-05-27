# EpochMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **string** |  | [default to undefined]
**campaignName** | **string** | Name of the campaign. | [default to undefined]
**epochId** | **number** | Epoch Id of the epoch. | [default to undefined]
**epochNumber** | **number** | Epoch number for the queried epoch. | [default to undefined]
**startDate** | **number** | Time in milliseconds for campaign start date. | [default to undefined]
**endDate** | **number** | Time in milliseconds for campaign end date. | [default to undefined]

## Example

```typescript
import { EpochMetadata } from '@bluefin/api-client';

const instance: EpochMetadata = {
    status,
    campaignName,
    epochId,
    epochNumber,
    startDate,
    endDate,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
