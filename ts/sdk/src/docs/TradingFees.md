# TradingFees


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**makerFeeE9** | **string** | The Account Maker Fee (e9 format). | [default to undefined]
**takerFeeE9** | **string** | The Account Taker Fee (e9 format). | [default to undefined]
**isApplied** | **boolean** | Are the fees applied on the account? | [default to undefined]

## Example

```typescript
import { TradingFees } from '@bluefin/api-client';

const instance: TradingFees = {
    makerFeeE9,
    takerFeeE9,
    isApplied,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
