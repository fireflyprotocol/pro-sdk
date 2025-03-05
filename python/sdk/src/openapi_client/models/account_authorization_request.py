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

from pydantic import BaseModel, ConfigDict, Field, StrictStr
from typing import Any, ClassVar, Dict, List
from openapi_client.models.account_authorization_request_signed_fields import AccountAuthorizationRequestSignedFields
from typing import Optional, Set
from typing_extensions import Self

class AccountAuthorizationRequest(BaseModel):
    """
    AccountAuthorizationRequest
    """ # noqa: E501
    signed_fields: AccountAuthorizationRequestSignedFields = Field(alias="signedFields")
    signature: StrictStr = Field(description="The signature of the request, encoded from the signedFields")
    request_hash: StrictStr = Field(description="Used to uniquely identify the request. Created by hex encoding the bcs encoded signedFields.", alias="requestHash")
    __properties: ClassVar[List[str]] = ["signedFields", "signature", "requestHash"]

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
        """Create an instance of AccountAuthorizationRequest from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of signed_fields
        if self.signed_fields:
            _dict['signedFields'] = self.signed_fields.to_dict()
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of AccountAuthorizationRequest from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "signedFields": AccountAuthorizationRequestSignedFields.from_dict(obj["signedFields"]) if obj.get("signedFields") is not None else None,
            "signature": obj.get("signature"),
            "requestHash": obj.get("requestHash")
        })
        return _obj


