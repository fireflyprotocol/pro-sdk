# CreateOrderRequestSignedFields


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | **str** | The symbol of the perpetual for which to create the order | 
**account_address** | **str** | The account address of the order. May be an account user is authorized for. | 
**price_e9** | **str** | The price in base e9 of the asset to be traded. Should always be a number | 
**quantity_e9** | **str** | The quantity in base e9 of the asset to be traded. Should always be a number | 
**side** | [**OrderSide**](OrderSide.md) |  | 
**leverage_e9** | **str** | The leverage in base e9 of the order to be traded. Should always be a number | 
**is_isolated** | **bool** | Is this order isolated or cross margin. Note market must be set to the same mode. | [default to False]
**salt** | **str** | The random generated SALT. Should always be a number | 
**ids_id** | **str** | the ID of the internal datastore for the target network | 
**expires_at_millis** | **int** | timestamp in millis at which order will expire. Defaults to 1 month for LIMIT orders if not provided | 
**signed_at_millis** | **int** | The timestamp in millis at which the request was signed | 

## Example

```python
from openapi_client.models.create_order_request_signed_fields import CreateOrderRequestSignedFields

# TODO update the JSON string below
json = "{}"
# create an instance of CreateOrderRequestSignedFields from a JSON string
create_order_request_signed_fields_instance = CreateOrderRequestSignedFields.from_json(json)
# print the JSON string representation of the object
print(CreateOrderRequestSignedFields.to_json())

# convert the object into a dict
create_order_request_signed_fields_dict = create_order_request_signed_fields_instance.to_dict()
# create an instance of CreateOrderRequestSignedFields from a dict
create_order_request_signed_fields_from_dict = CreateOrderRequestSignedFields.from_dict(create_order_request_signed_fields_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


