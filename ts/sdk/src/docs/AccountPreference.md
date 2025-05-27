# AccountPreference


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**language** | **string** | User preferred language. | [optional] [default to undefined]
**theme** | **string** | User preferred theme. | [optional] [default to undefined]
**market** | [**Array&lt;AccountMarketPreference&gt;**](AccountMarketPreference.md) |  | [optional] [default to undefined]

## Example

```typescript
import { AccountPreference } from '@bluefin/api-client';

const instance: AccountPreference = {
    language,
    theme,
    market,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
