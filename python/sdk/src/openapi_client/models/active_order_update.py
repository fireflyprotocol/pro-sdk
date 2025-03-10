# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from openapi_client.models.order_status1 import OrderStatus1
from openapi_client.models.order_time_in_force1 import OrderTimeInForce1
from openapi_client.models.order_type1 import OrderType1
from openapi_client.models.self_trade_prevention_type1 import SelfTradePreventionType1
from openapi_client.models.side import Side
from typing import Optional, Set
from typing_extensions import Self

class ActiveOrderUpdate(BaseModel):
    """
    Information about an order update.
    """ # noqa: E501
    order_hash: StrictStr = Field(description="The unique hash of the order.", alias="orderHash")
    client_order_id: Optional[StrictStr] = Field(default=None, description="The client-provided order ID.", alias="clientOrderId")
    symbol: StrictStr = Field(description="The symbol of the market.")
    account_address: StrictStr = Field(description="The address of the account.", alias="accountAddress")
    price_e9: StrictStr = Field(description="The price of the order in scientific notation with 9 decimal places.", alias="priceE9")
    quantity_e9: StrictStr = Field(description="The quantity of the order in scientific notation with 9 decimal places.", alias="quantityE9")
    filled_quantity_e9: StrictStr = Field(description="The filled quantity of the order in scientific notation with 9 decimal places.", alias="filledQuantityE9")
    side: Side
    leverage_e9: StrictStr = Field(description="The leverage of the order in scientific notation with 9 decimal places.", alias="leverageE9")
    is_isolated: StrictBool = Field(description="Indicates if the order is isolated.", alias="isIsolated")
    salt: StrictStr = Field(description="A unique salt for the order.")
    expires_at_utc_millis: StrictInt = Field(description="The expiration timestamp of the order in milliseconds.", alias="expiresAtUtcMillis")
    signed_at_utc_millis: StrictInt = Field(description="The signing timestamp of the order in milliseconds.", alias="signedAtUtcMillis")
    type: OrderType1
    reduce_only: StrictBool = Field(description="Indicates if the order is reduce-only.", alias="reduceOnly")
    post_only: StrictBool = Field(description="Indicates if the order is post-only.", alias="postOnly")
    time_in_force: OrderTimeInForce1 = Field(alias="timeInForce")
    trigger_price_e9: Optional[StrictStr] = Field(default=None, description="The trigger price for stop-limit or stop-market orders.", alias="triggerPriceE9")
    status: OrderStatus1
    self_trade_prevention_type: SelfTradePreventionType1 = Field(alias="selfTradePreventionType")
    created_at_utc_millis: StrictInt = Field(description="The timestamp when the order was placed, in milliseconds.", alias="createdAtUtcMillis")
    updated_at_utc_millis: StrictInt = Field(description="The timestamp of the last update of the order in milliseconds.", alias="updatedAtUtcMillis")
    __properties: ClassVar[List[str]] = ["orderHash", "clientOrderId", "symbol", "accountAddress", "priceE9", "quantityE9", "filledQuantityE9", "side", "leverageE9", "isIsolated", "salt", "expiresAtUtcMillis", "signedAtUtcMillis", "type", "reduceOnly", "postOnly", "timeInForce", "triggerPriceE9", "status", "selfTradePreventionType", "createdAtUtcMillis", "updatedAtUtcMillis"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of ActiveOrderUpdate from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of ActiveOrderUpdate from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "orderHash": obj.get("orderHash"),
            "clientOrderId": obj.get("clientOrderId"),
            "symbol": obj.get("symbol"),
            "accountAddress": obj.get("accountAddress"),
            "priceE9": obj.get("priceE9"),
            "quantityE9": obj.get("quantityE9"),
            "filledQuantityE9": obj.get("filledQuantityE9"),
            "side": obj.get("side"),
            "leverageE9": obj.get("leverageE9"),
            "isIsolated": obj.get("isIsolated"),
            "salt": obj.get("salt"),
            "expiresAtUtcMillis": obj.get("expiresAtUtcMillis"),
            "signedAtUtcMillis": obj.get("signedAtUtcMillis"),
            "type": obj.get("type"),
            "reduceOnly": obj.get("reduceOnly"),
            "postOnly": obj.get("postOnly"),
            "timeInForce": obj.get("timeInForce"),
            "triggerPriceE9": obj.get("triggerPriceE9"),
            "status": obj.get("status"),
            "selfTradePreventionType": obj.get("selfTradePreventionType"),
            "createdAtUtcMillis": obj.get("createdAtUtcMillis"),
            "updatedAtUtcMillis": obj.get("updatedAtUtcMillis")
        })
        return _obj


