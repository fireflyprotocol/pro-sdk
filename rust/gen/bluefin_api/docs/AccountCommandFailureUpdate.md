# AccountCommandFailureUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reason** | **String** | The reason for the failure. | 
**reason_code** | Option<[**models::CommandFailureReasonCode**](CommandFailureReasonCode.md)> |  | [optional]
**failed_command_type** | **String** | The type of command that failed. | 
**failed_command_type_code** | Option<[**models::FailedCommandType**](FailedCommandType.md)> |  | [optional]
**failed_at_millis** | **u64** | The timestamp when the command failed in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


