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

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictFloat, StrictInt, StrictStr, field_validator
from typing import Any, ClassVar, Dict, List, Optional, Union
from openapi_client.models.position_side import PositionSide
from openapi_client.models.trade_side import TradeSide
from openapi_client.models.trade_type import TradeType
from typing import Optional, Set
from typing_extensions import Self

class Trade(BaseModel):
    """
    Trade
    """ # noqa: E501
    id: StrictStr = Field(description="Trade ID")
    client_order_id: Optional[StrictStr] = Field(default=None, description="Client order ID.", alias="clientOrderId")
    symbol: Optional[StrictStr] = Field(default=None, description="Market address.")
    order_hash: Optional[StrictStr] = Field(default=None, description="Order hash.", alias="orderHash")
    trade_type: Optional[TradeType] = Field(default=None, alias="tradeType")
    side: TradeSide
    is_maker: Optional[StrictBool] = Field(default=None, description="Indicates if the user was a maker to the trade.", alias="isMaker")
    price_e9: StrictStr = Field(description="Trade price (e9 format).", alias="priceE9")
    quantity_e9: StrictStr = Field(description="Trade quantity (e9 format).", alias="quantityE9")
    quote_quantity_e9: StrictStr = Field(description="Quote quantity (e9 format).", alias="quoteQuantityE9")
    realized_pnl_e9: Optional[StrictStr] = Field(default=None, description="Realized profit and loss (e9 format).", alias="realizedPnlE9")
    position_side: Optional[PositionSide] = Field(default=None, alias="positionSide")
    trading_fee_e9: Optional[StrictStr] = Field(default=None, description="Trading fee (e9 format).", alias="tradingFeeE9")
    trading_fee_asset: Optional[StrictStr] = Field(default=None, description="Asset used for trading fee.", alias="tradingFeeAsset")
    gas_fee_e9: Optional[Union[StrictFloat, StrictInt]] = Field(default=None, description="Gas fee.", alias="gasFeeE9")
    gas_fee_asset: Optional[StrictStr] = Field(default=None, description="Asset used for gas fee.", alias="gasFeeAsset")
    executed_at_millis: StrictInt = Field(description="Trade timestamp in milliseconds since Unix epoch.", alias="executedAtMillis")
    __properties: ClassVar[List[str]] = ["id", "clientOrderId", "symbol", "orderHash", "tradeType", "side", "isMaker", "priceE9", "quantityE9", "quoteQuantityE9", "realizedPnlE9", "positionSide", "tradingFeeE9", "tradingFeeAsset", "gasFeeE9", "gasFeeAsset", "executedAtMillis"]

    @field_validator('trading_fee_asset')
    def trading_fee_asset_validate_enum(cls, value):
        """Validates the enum"""
        if value is None:
            return value

        if value not in set(['USDC', 'BLUE', 'UNSPECIFIED']):
            raise ValueError("must be one of enum values ('USDC', 'BLUE', 'UNSPECIFIED')")
        return value

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
        """Create an instance of Trade from a JSON string"""
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
        """Create an instance of Trade from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "clientOrderId": obj.get("clientOrderId"),
            "symbol": obj.get("symbol"),
            "orderHash": obj.get("orderHash"),
            "tradeType": obj.get("tradeType"),
            "side": obj.get("side"),
            "isMaker": obj.get("isMaker"),
            "priceE9": obj.get("priceE9"),
            "quantityE9": obj.get("quantityE9"),
            "quoteQuantityE9": obj.get("quoteQuantityE9"),
            "realizedPnlE9": obj.get("realizedPnlE9"),
            "positionSide": obj.get("positionSide"),
            "tradingFeeE9": obj.get("tradingFeeE9"),
            "tradingFeeAsset": obj.get("tradingFeeAsset"),
            "gasFeeE9": obj.get("gasFeeE9"),
            "gasFeeAsset": obj.get("gasFeeAsset"),
            "executedAtMillis": obj.get("executedAtMillis")
        })
        return _obj


