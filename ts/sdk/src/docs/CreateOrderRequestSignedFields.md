# CreateOrderRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **string** | The symbol of the perpetual for which to create the order | [default to undefined]
**accountAddress** | **string** | The account address of the order. May be an account user is authorized for. | [default to undefined]
**priceE9** | **string** | The price in base e9 of the asset to be traded. Should always be a number | [default to undefined]
**quantityE9** | **string** | The quantity in base e9 of the asset to be traded. Should always be a number | [default to undefined]
**side** | [**OrderSide**](OrderSide.md) |  | [default to undefined]
**leverageE9** | **string** | The leverage in base e9 of the order to be traded. Should always be a number | [default to undefined]
**isIsolated** | **boolean** | Is this order isolated or cross margin. Note market must be set to the same mode. | [default to false]
**salt** | **string** | The random generated SALT. Should always be a number | [default to undefined]
**idsId** | **string** | the ID of the internal datastore for the target network | [default to undefined]
**expiresAtMillis** | **number** | The timestamp in millis at which order will expire. | [default to undefined]
**signedAtMillis** | **number** | The timestamp in millis at which the request was signed | [default to undefined]

## Example

```typescript
import { CreateOrderRequestSignedFields } from '@bluefin/api-client';

const instance: CreateOrderRequestSignedFields = {
    symbol,
    accountAddress,
    priceE9,
    quantityE9,
    side,
    leverageE9,
    isIsolated,
    salt,
    idsId,
    expiresAtMillis,
    signedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
