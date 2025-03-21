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
from openapi_client.models.side import Side
from openapi_client.models.trade_type import TradeType
from typing import Optional, Set
from typing_extensions import Self

class AccountTradeUpdate(BaseModel):
    """
    Details about a trade in the account.
    """ # noqa: E501
    trade_id: StrictStr = Field(description="The unique identifier for the trade.", alias="tradeId")
    client_order_id: Optional[StrictStr] = Field(default=None, description="The client order ID associated with the trade.", alias="clientOrderId")
    symbol: StrictStr = Field(description="The symbol of the market.")
    order_hash: StrictStr = Field(description="The hash of the order.", alias="orderHash")
    type: TradeType
    trade_side: Side = Field(alias="tradeSide")
    is_maker: StrictBool = Field(description="Indicates if the trade was a maker order.", alias="isMaker")
    price_e9: StrictStr = Field(description="The price of the trade.", alias="priceE9")
    quantity_e9: StrictStr = Field(description="The quantity of the trade.", alias="quantityE9")
    quote_quantity_e9: StrictStr = Field(description="The quote quantity of the trade.", alias="quoteQuantityE9")
    realized_pnl_e9: StrictStr = Field(description="The realized profit and loss.", alias="realizedPnlE9")
    position_side: Side = Field(alias="positionSide")
    trading_fee_e9: StrictStr = Field(description="The trading fee for the trade.", alias="tradingFeeE9")
    trading_fee_asset_symbol: StrictStr = Field(description="The market symbol of the asset used for the trading fee.", alias="tradingFeeAssetSymbol")
    executed_at_millis: StrictInt = Field(description="The timestamp when the trade was executed in milliseconds.", alias="executedAtMillis")
    __properties: ClassVar[List[str]] = ["tradeId", "clientOrderId", "symbol", "orderHash", "type", "tradeSide", "isMaker", "priceE9", "quantityE9", "quoteQuantityE9", "realizedPnlE9", "positionSide", "tradingFeeE9", "tradingFeeAssetSymbol", "executedAtMillis"]

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
        """Create an instance of AccountTradeUpdate from a JSON string"""
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
        """Create an instance of AccountTradeUpdate from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "tradeId": obj.get("tradeId"),
            "clientOrderId": obj.get("clientOrderId"),
            "symbol": obj.get("symbol"),
            "orderHash": obj.get("orderHash"),
            "type": obj.get("type"),
            "tradeSide": obj.get("tradeSide"),
            "isMaker": obj.get("isMaker"),
            "priceE9": obj.get("priceE9"),
            "quantityE9": obj.get("quantityE9"),
            "quoteQuantityE9": obj.get("quoteQuantityE9"),
            "realizedPnlE9": obj.get("realizedPnlE9"),
            "positionSide": obj.get("positionSide"),
            "tradingFeeE9": obj.get("tradingFeeE9"),
            "tradingFeeAssetSymbol": obj.get("tradingFeeAssetSymbol"),
            "executedAtMillis": obj.get("executedAtMillis")
        })
        return _obj


