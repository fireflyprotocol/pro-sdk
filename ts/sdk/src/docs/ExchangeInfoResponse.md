# ExchangeInfoResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**Array&lt;AssetConfig&gt;**](AssetConfig.md) | List of assets available on the exchange. | [default to undefined]
**contractsConfig** | [**ContractsConfig**](ContractsConfig.md) |  | [optional] [default to undefined]
**markets** | [**Array&lt;Market&gt;**](Market.md) | List of markets available on the exchange. | [default to undefined]
**tradingGasFeeE9** | **string** | Current gas fee set for subsidized trades (e9 format) | [default to undefined]
**serverTimeAtMillis** | **number** | Server time in milliseconds. | [default to undefined]
**timezone** | **string** | Timezone of the exchange. | [default to undefined]

## Example

```typescript
import { ExchangeInfoResponse } from '@bluefin/api-client';

const instance: ExchangeInfoResponse = {
    assets,
    contractsConfig,
    markets,
    tradingGasFeeE9,
    serverTimeAtMillis,
    timezone,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
