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


class OrderTimeInForce(str, Enum):
    """
    Specify order execution, and by default, all orders are GTT. UNSPECIFIED is set to default
    """

    """
    allowed enum values
    """
    GTT = 'GTT'
    IOC = 'IOC'
    FOK = 'FOK'
    UNSPECIFIED = 'UNSPECIFIED'

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Create an instance of OrderTimeInForce from a JSON string"""
        return cls(json.loads(json_str))


