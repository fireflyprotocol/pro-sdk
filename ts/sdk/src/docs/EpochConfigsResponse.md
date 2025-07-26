# EpochConfigsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maxIntervalNumber** | **number** | The maximum interval number available. | [default to undefined]
**intervalNumber** | **number** | The current interval number being queried. | [default to undefined]
**data** | [**Array&lt;EpochConfigs&gt;**](EpochConfigs.md) | List of epoch configs for different campaigns. | [default to undefined]

## Example

```typescript
import { EpochConfigsResponse } from '@bluefin/api-client';

const instance: EpochConfigsResponse = {
    maxIntervalNumber,
    intervalNumber,
    data,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
