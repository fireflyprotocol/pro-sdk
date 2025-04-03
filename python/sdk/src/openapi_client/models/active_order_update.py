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
from openapi_client.models.order_status import OrderStatus
from openapi_client.models.order_time_in_force import OrderTimeInForce
from openapi_client.models.order_type import OrderType
from openapi_client.models.self_trade_prevention_type import SelfTradePreventionType
from openapi_client.models.trade_side import TradeSide
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
    side: TradeSide
    leverage_e9: StrictStr = Field(description="The leverage of the order in scientific notation with 9 decimal places.", alias="leverageE9")
    is_isolated: StrictBool = Field(description="Indicates if the order is isolated.", alias="isIsolated")
    salt: StrictStr = Field(description="A unique salt for the order.")
    expires_at_millis: StrictInt = Field(description="The expiration timestamp of the order in milliseconds.", alias="expiresAtMillis")
    signed_at_millis: StrictInt = Field(description="The signing timestamp of the order in milliseconds.", alias="signedAtMillis")
    type: OrderType
    reduce_only: StrictBool = Field(description="Indicates if the order is reduce-only.", alias="reduceOnly")
    post_only: StrictBool = Field(description="Indicates if the order is post-only.", alias="postOnly")
    time_in_force: OrderTimeInForce = Field(alias="timeInForce")
    trigger_price_e9: Optional[StrictStr] = Field(default=None, description="The trigger price for stop-limit or stop-market orders.", alias="triggerPriceE9")
    status: OrderStatus
    self_trade_prevention_type: SelfTradePreventionType = Field(alias="selfTradePreventionType")
    created_at_millis: StrictInt = Field(description="The timestamp when the order was placed, in milliseconds.", alias="createdAtMillis")
    updated_at_millis: StrictInt = Field(description="The timestamp of the last update of the order in milliseconds.", alias="updatedAtMillis")
    __properties: ClassVar[List[str]] = ["orderHash", "clientOrderId", "symbol", "accountAddress", "priceE9", "quantityE9", "filledQuantityE9", "side", "leverageE9", "isIsolated", "salt", "expiresAtMillis", "signedAtMillis", "type", "reduceOnly", "postOnly", "timeInForce", "triggerPriceE9", "status", "selfTradePreventionType", "createdAtMillis", "updatedAtMillis"]

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
            "expiresAtMillis": obj.get("expiresAtMillis"),
            "signedAtMillis": obj.get("signedAtMillis"),
            "type": obj.get("type"),
            "reduceOnly": obj.get("reduceOnly"),
            "postOnly": obj.get("postOnly"),
            "timeInForce": obj.get("timeInForce") if obj.get("timeInForce") is not None else OrderTimeInForce.GTT,
            "triggerPriceE9": obj.get("triggerPriceE9"),
            "status": obj.get("status"),
            "selfTradePreventionType": obj.get("selfTradePreventionType") if obj.get("selfTradePreventionType") is not None else SelfTradePreventionType.MAKER,
            "createdAtMillis": obj.get("createdAtMillis"),
            "updatedAtMillis": obj.get("updatedAtMillis")
        })
        return _obj


