# KoraIntervalMetadata


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **string** |  | [default to undefined]
**startDate** | **number** | Time in milliseconds for interval start date | [default to undefined]
**endDate** | **number** | Time in milliseconds for interval end date | [default to undefined]
**intervalId** | **number** | Interval ID | [default to undefined]
**intervalType** | **string** | Type of the interval | [optional] [default to undefined]
**protocol** | **string** | Protocol for the interval | [optional] [default to undefined]

## Example

```typescript
import { KoraIntervalMetadata } from '@bluefin/api-client';

const instance: KoraIntervalMetadata = {
    status,
    startDate,
    endDate,
    intervalId,
    intervalType,
    protocol,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
