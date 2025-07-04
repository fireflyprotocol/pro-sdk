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

from pydantic import BaseModel, ConfigDict, Field, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from openapi_client.models.transaction_type import TransactionType
from typing import Optional, Set
from typing_extensions import Self

class Transaction(BaseModel):
    """
    Transaction
    """ # noqa: E501
    id: StrictStr = Field(description="Transaction ID.")
    symbol: Optional[StrictStr] = Field(default=None, description="Market address.")
    type: TransactionType
    amount_e9: StrictStr = Field(description="Amount in e9 format (positive or negative).", alias="amountE9")
    status: Optional[StrictStr] = Field(default=None, description="Transaction status (SUCCESS, REJECTED).")
    asset_symbol: StrictStr = Field(description="Asset bank address.", alias="assetSymbol")
    trade_id: Optional[StrictStr] = Field(default=None, description="Trade ID", alias="tradeId")
    executed_at_millis: StrictInt = Field(description="Transaction timestamp in milliseconds since Unix epoch.", alias="executedAtMillis")
    __properties: ClassVar[List[str]] = ["id", "symbol", "type", "amountE9", "status", "assetSymbol", "tradeId", "executedAtMillis"]

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
        """Create an instance of Transaction from a JSON string"""
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
        """Create an instance of Transaction from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "symbol": obj.get("symbol"),
            "type": obj.get("type"),
            "amountE9": obj.get("amountE9"),
            "status": obj.get("status"),
            "assetSymbol": obj.get("assetSymbol"),
            "tradeId": obj.get("tradeId"),
            "executedAtMillis": obj.get("executedAtMillis")
        })
        return _obj


