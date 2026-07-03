# UpdateAccountPreferenceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**favorites** | Option<**Vec<String>**> | Favorite market symbols. Send the full array each time; to remove a favorite, send the array without it.  | [optional]
**function_bar_mode** | Option<**String**> | Function bar display mode. Note the mixed casing: `all` is lowercase while `Popular` and `Favorites` are capitalized.  | [optional]
**onboarding_completed** | Option<**bool**> | Whether the user has completed onboarding. | [optional]
**terms_accepted** | Option<**bool**> | Whether the user has accepted the terms of service. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


