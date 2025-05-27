# AccountStreamMessage

Account stream message for account-related events. The payload depends on the event type. 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | [**AccountEventType**](AccountEventType.md) |  | [default to undefined]
**reason** | [**AccountEventReason**](AccountEventReason.md) |  | [default to undefined]
**payload** | [**AccountStreamMessagePayload**](AccountStreamMessagePayload.md) |  | [default to undefined]

## Example

```typescript
import { AccountStreamMessage } from '@bluefin/api-client';

const instance: AccountStreamMessage = {
    event,
    reason,
    payload,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
