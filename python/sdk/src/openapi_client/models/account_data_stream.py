# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import json
from enum import Enum
from typing_extensions import Self


class AccountDataStream(str, Enum):
    """
    Represents the type of account data stream.
    """

    """
    allowed enum values
    """
    ACCOUNTORDERUPDATE = 'AccountOrderUpdate'
    ACCOUNTTRADEUPDATE = 'AccountTradeUpdate'
    ACCOUNTPOSITIONUPDATE = 'AccountPositionUpdate'
    ACCOUNTUPDATE = 'AccountUpdate'
    ACCOUNTTRANSACTIONUPDATE = 'AccountTransactionUpdate'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of AccountDataStream from a JSON string"""
        return cls(json.loads(json_str))


