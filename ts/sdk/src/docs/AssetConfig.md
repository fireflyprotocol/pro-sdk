# AssetConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assetType** | **string** | The bank address of the asset. | [default to undefined]
**symbol** | **string** | Asset symbol. | [default to undefined]
**decimals** | **number** | Default precision for rendering this asset. | [default to undefined]
**weight** | **string** | Weight applied to asset to use as margin in Multi-Assets mode. | [default to undefined]
**marginAvailable** | **boolean** | Indicates if the asset can be used as margin in Multi-Assets mode. | [default to undefined]

## Example

```typescript
import { AssetConfig } from '@bluefin/api-client';

const instance: AssetConfig = {
    assetType,
    symbol,
    decimals,
    weight,
    marginAvailable,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
