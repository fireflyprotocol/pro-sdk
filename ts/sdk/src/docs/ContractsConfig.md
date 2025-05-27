# ContractsConfig

Contract configuration for the exchange.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**edsId** | **string** | External Data Store Address | [default to undefined]
**idsId** | **string** | External Data Store Address | [default to undefined]
**network** | **string** | Network environment | [default to undefined]
**baseContractAddress** | **string** | Base contract address | [default to undefined]
**currentContractAddress** | **string** | Current contract address | [default to undefined]
**operators** | [**Operators**](Operators.md) |  | [default to undefined]

## Example

```typescript
import { ContractsConfig } from '@bluefin/api-client';

const instance: ContractsConfig = {
    edsId,
    idsId,
    network,
    baseContractAddress,
    currentContractAddress,
    operators,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
