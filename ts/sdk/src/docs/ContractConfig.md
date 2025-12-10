# ContractConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**adminCap** | **string** |  | [optional] [default to undefined]
**_package** | **string** |  | [optional] [default to undefined]
**upgradeCap** | **string** |  | [optional] [default to undefined]
**supportedCoin** | **string** |  | [optional] [default to undefined]
**bluefinBank** | **string** |  | [optional] [default to undefined]
**bluefinSequencer** | **string** |  | [optional] [default to undefined]
**bluefinSubAccounts** | **string** |  | [optional] [default to undefined]
**bluefinVaultStore** | **string** |  | [optional] [default to undefined]
**bluefinPackageBase** | **string** |  | [optional] [default to undefined]
**bluefinPackage** | **string** |  | [optional] [default to undefined]
**rewardsPool** | [**{ [key: string]: RewardsPoolEntry | undefined; }**](RewardsPoolEntry.md) |  | [optional] [default to undefined]

## Example

```typescript
import { ContractConfig } from '@bluefin/api-client';

const instance: ContractConfig = {
    adminCap,
    _package,
    upgradeCap,
    supportedCoin,
    bluefinBank,
    bluefinSequencer,
    bluefinSubAccounts,
    bluefinVaultStore,
    bluefinPackageBase,
    bluefinPackage,
    rewardsPool,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
