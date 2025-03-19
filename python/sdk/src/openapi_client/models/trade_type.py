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


class TradeType(str, Enum):
    """
    Type of trade.
    """

    """
    allowed enum values
    """
    ORDER = 'ORDER'
    LIQUIDATION = 'LIQUIDATION'
    DELEVERAGE = 'DELEVERAGE'
    UNSPECIFIED = 'UNSPECIFIED'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of TradeType from a JSON string"""
        return cls(json.loads(json_str))


