# UpdateAccountPreferenceRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**favorites** | **Array&lt;string&gt;** | Favorite market symbols. Send the full array each time; to remove a favorite, send the array without it.  | [optional] [default to undefined]
**functionBarMode** | **string** | Function bar display mode. Note the mixed casing: &#x60;all&#x60; is lowercase while &#x60;Popular&#x60; and &#x60;Favorites&#x60; are capitalized.  | [optional] [default to undefined]
**onboardingCompleted** | **boolean** | Whether the user has completed onboarding. | [optional] [default to undefined]
**termsAccepted** | **boolean** | Whether the user has accepted the terms of service. | [optional] [default to undefined]

## Example

```typescript
import { UpdateAccountPreferenceRequest } from '@bluefin/api-client';

const instance: UpdateAccountPreferenceRequest = {
    favorites,
    functionBarMode,
    onboardingCompleted,
    termsAccepted,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
