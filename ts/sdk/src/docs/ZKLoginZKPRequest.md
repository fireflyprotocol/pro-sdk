# ZKLoginZKPRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network** | **string** | The network to use (e.g., \&quot;mainnet\&quot;, \&quot;testnet\&quot;). | [optional] [default to undefined]
**ephemeralPublicKey** | **string** | The ephemeral public key for the ZK proof. | [default to undefined]
**maxEpoch** | **number** | The maximum epoch for the ZK proof. | [default to undefined]
**randomness** | **string** | Randomness value for the ZK proof. | [default to undefined]

## Example

```typescript
import { ZKLoginZKPRequest } from '@bluefin/api-client';

const instance: ZKLoginZKPRequest = {
    network,
    ephemeralPublicKey,
    maxEpoch,
    randomness,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
