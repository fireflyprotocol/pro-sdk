# OrderTwapConfig

Configuration for Time-Weighted Average Price orders

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subOrdersCount** | **string** | Number of sub-orders to split the total quantity into | [optional] [default to undefined]
**runningTimeInMinutes** | **string** | Total time in minutes over which to execute the TWAP order | [optional] [default to undefined]

## Example

```typescript
import { OrderTwapConfig } from '@bluefin/api-client';

const instance: OrderTwapConfig = {
    subOrdersCount,
    runningTimeInMinutes,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
