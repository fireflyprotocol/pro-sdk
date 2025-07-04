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
from typing import Optional, Set
from typing_extensions import Self

class AffiliateLeaderDashboard(BaseModel):
    """
    AffiliateLeaderDashboard
    """ # noqa: E501
    user_address: StrictStr = Field(description="The user's wallet address", alias="userAddress")
    name: Optional[StrictStr] = Field(default=None, description="Name of the affiliate")
    parent_address: StrictStr = Field(description="The parent affiliate's wallet address", alias="parentAddress")
    parent_name: Optional[StrictStr] = Field(default=None, description="Name of the parent affiliate", alias="parentName")
    perps_rank: StrictInt = Field(description="Ranking in perps trading category", alias="perpsRank")
    spot_rank: StrictInt = Field(description="Ranking in spot trading category", alias="spotRank")
    lending_rank: StrictInt = Field(description="Ranking in lending category", alias="lendingRank")
    perps_total_earnings_e9: StrictStr = Field(description="Total earnings from perps trading (e9 format)", alias="perpsTotalEarningsE9")
    spot_total_earnings_e9: StrictStr = Field(description="Total earnings from spot trading (e9 format)", alias="spotTotalEarningsE9")
    lending_total_earnings_e9: StrictStr = Field(description="Total earnings from lending (e9 format)", alias="lendingTotalEarningsE9")
    __properties: ClassVar[List[str]] = ["userAddress", "name", "parentAddress", "parentName", "perpsRank", "spotRank", "lendingRank", "perpsTotalEarningsE9", "spotTotalEarningsE9", "lendingTotalEarningsE9"]

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
        """Create an instance of AffiliateLeaderDashboard from a JSON string"""
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
        """Create an instance of AffiliateLeaderDashboard from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "userAddress": obj.get("userAddress"),
            "name": obj.get("name"),
            "parentAddress": obj.get("parentAddress"),
            "parentName": obj.get("parentName"),
            "perpsRank": obj.get("perpsRank"),
            "spotRank": obj.get("spotRank"),
            "lendingRank": obj.get("lendingRank"),
            "perpsTotalEarningsE9": obj.get("perpsTotalEarningsE9"),
            "spotTotalEarningsE9": obj.get("spotTotalEarningsE9"),
            "lendingTotalEarningsE9": obj.get("lendingTotalEarningsE9")
        })
        return _obj


