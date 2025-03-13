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

class AccountTransactionUpdate(BaseModel):
    """
    Details about a transaction in the account.
    """ # noqa: E501
    symbol: Optional[StrictStr] = Field(default=None, description="The symbol of the market.")
    transaction_type: TransactionType = Field(alias="transactionType")
    amount_e9: StrictStr = Field(description="The amount of the transaction in scientific notation with 9 decimal places.", alias="amountE9")
    asset_symbol: Optional[StrictStr] = Field(default=None, description="The symbol of the asset.", alias="assetSymbol")
    trade_id: Optional[StrictStr] = Field(default=None, description="The trade ID associated with the transaction.", alias="tradeId")
    executed_at_millis: StrictInt = Field(description="The timestamp when the transaction was executed in milliseconds.", alias="executedAtMillis")
    __properties: ClassVar[List[str]] = ["symbol", "transactionType", "amountE9", "assetSymbol", "tradeId", "executedAtMillis"]

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
        """Create an instance of AccountTransactionUpdate from a JSON string"""
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
        """Create an instance of AccountTransactionUpdate from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "symbol": obj.get("symbol"),
            "transactionType": obj.get("transactionType"),
            "amountE9": obj.get("amountE9"),
            "assetSymbol": obj.get("assetSymbol"),
            "tradeId": obj.get("tradeId"),
            "executedAtMillis": obj.get("executedAtMillis")
        })
        return _obj


