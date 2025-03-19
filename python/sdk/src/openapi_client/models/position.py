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
from typing import Any, ClassVar, Dict, List
from openapi_client.models.position_side import PositionSide
from typing import Optional, Set
from typing_extensions import Self

class Position(BaseModel):
    """
    Position
    """ # noqa: E501
    symbol: StrictStr = Field(description="Market address.")
    avg_entry_price_e9: StrictStr = Field(description="Average entry price determined by a simple average of all entry prices resulting in this position size (e9 format).", alias="avgEntryPriceE9")
    leverage_e9: StrictStr = Field(description="Isolated position leverage (e9 format).", alias="leverageE9")
    liquidation_price_e9: StrictStr = Field(description="Liquidation price (e9 format).", alias="liquidationPriceE9")
    mark_price_e9: StrictStr = Field(description="Mark price (e9 format).", alias="markPriceE9")
    notional_value_e9: StrictStr = Field(description="Notional value (e9 format).", alias="notionalValueE9")
    size_e9: StrictStr = Field(description="Position size (e9 format).", alias="sizeE9")
    unrealized_pnl_e9: StrictStr = Field(description="Unrealized profit (e9 format).", alias="unrealizedPnlE9")
    side: PositionSide
    initial_margin_e9: StrictStr = Field(description="Initial margin required with current mark price (e9 format).", alias="initialMarginE9")
    maintenance_margin_e9: StrictStr = Field(description="Maintenance margin required with current mark price (e9 format).", alias="maintenanceMarginE9")
    is_isolated: StrictBool = Field(description="If the position is isolated.", alias="isIsolated")
    isolated_margin_e9: StrictStr = Field(description="Margin value present if margin type is isolated (e9 format).", alias="isolatedMarginE9")
    updated_at_millis: StrictInt = Field(description="Last update time.", alias="updatedAtMillis")
    __properties: ClassVar[List[str]] = ["symbol", "avgEntryPriceE9", "leverageE9", "liquidationPriceE9", "markPriceE9", "notionalValueE9", "sizeE9", "unrealizedPnlE9", "side", "initialMarginE9", "maintenanceMarginE9", "isIsolated", "isolatedMarginE9", "updatedAtMillis"]

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
        """Create an instance of Position from a JSON string"""
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
        """Create an instance of Position from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "symbol": obj.get("symbol"),
            "avgEntryPriceE9": obj.get("avgEntryPriceE9"),
            "leverageE9": obj.get("leverageE9"),
            "liquidationPriceE9": obj.get("liquidationPriceE9"),
            "markPriceE9": obj.get("markPriceE9"),
            "notionalValueE9": obj.get("notionalValueE9"),
            "sizeE9": obj.get("sizeE9"),
            "unrealizedPnlE9": obj.get("unrealizedPnlE9"),
            "side": obj.get("side"),
            "initialMarginE9": obj.get("initialMarginE9"),
            "maintenanceMarginE9": obj.get("maintenanceMarginE9"),
            "isIsolated": obj.get("isIsolated"),
            "isolatedMarginE9": obj.get("isolatedMarginE9"),
            "updatedAtMillis": obj.get("updatedAtMillis")
        })
        return _obj


