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
import pprint
from pydantic import BaseModel, ConfigDict, Field, StrictStr, ValidationError, field_validator
from typing import Any, List, Optional
from openapi_client.models.account_aggregated_trade_update import AccountAggregatedTradeUpdate
from openapi_client.models.account_command_failure_update import AccountCommandFailureUpdate
from openapi_client.models.account_order_update import AccountOrderUpdate
from openapi_client.models.account_position_update import AccountPositionUpdate
from openapi_client.models.account_trade_update import AccountTradeUpdate
from openapi_client.models.account_transaction_update import AccountTransactionUpdate
from openapi_client.models.account_update import AccountUpdate
from pydantic import StrictStr, Field
from typing import Union, List, Set, Optional, Dict
from typing_extensions import Literal, Self

ACCOUNTSTREAMMESSAGEPAYLOAD_ONE_OF_SCHEMAS = ["AccountAggregatedTradeUpdate", "AccountCommandFailureUpdate", "AccountOrderUpdate", "AccountPositionUpdate", "AccountTradeUpdate", "AccountTransactionUpdate", "AccountUpdate"]

class AccountStreamMessagePayload(BaseModel):
    """
    The payload of the message, which varies based on the event type.
    """
    # data type: AccountUpdate
    oneof_schema_1_validator: Optional[AccountUpdate] = None
    # data type: AccountTradeUpdate
    oneof_schema_2_validator: Optional[AccountTradeUpdate] = None
    # data type: AccountAggregatedTradeUpdate
    oneof_schema_3_validator: Optional[AccountAggregatedTradeUpdate] = None
    # data type: AccountOrderUpdate
    oneof_schema_4_validator: Optional[AccountOrderUpdate] = None
    # data type: AccountTransactionUpdate
    oneof_schema_5_validator: Optional[AccountTransactionUpdate] = None
    # data type: AccountPositionUpdate
    oneof_schema_6_validator: Optional[AccountPositionUpdate] = None
    # data type: AccountCommandFailureUpdate
    oneof_schema_7_validator: Optional[AccountCommandFailureUpdate] = None
    actual_instance: Optional[Union[AccountAggregatedTradeUpdate, AccountCommandFailureUpdate, AccountOrderUpdate, AccountPositionUpdate, AccountTradeUpdate, AccountTransactionUpdate, AccountUpdate]] = None
    one_of_schemas: Set[str] = { "AccountAggregatedTradeUpdate", "AccountCommandFailureUpdate", "AccountOrderUpdate", "AccountPositionUpdate", "AccountTradeUpdate", "AccountTransactionUpdate", "AccountUpdate" }

    model_config = ConfigDict(
        validate_assignment=True,
        protected_namespaces=(),
    )


    def __init__(self, *args, **kwargs) -> None:
        if args:
            if len(args) > 1:
                raise ValueError("If a position argument is used, only 1 is allowed to set `actual_instance`")
            if kwargs:
                raise ValueError("If a position argument is used, keyword arguments cannot be used.")
            super().__init__(actual_instance=args[0])
        else:
            super().__init__(**kwargs)

    @field_validator('actual_instance')
    def actual_instance_must_validate_oneof(cls, v):
        instance = AccountStreamMessagePayload.model_construct()
        error_messages = []
        match = 0
        # validate data type: AccountUpdate
        if not isinstance(v, AccountUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountUpdate`")
        else:
            match += 1
        # validate data type: AccountTradeUpdate
        if not isinstance(v, AccountTradeUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountTradeUpdate`")
        else:
            match += 1
        # validate data type: AccountAggregatedTradeUpdate
        if not isinstance(v, AccountAggregatedTradeUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountAggregatedTradeUpdate`")
        else:
            match += 1
        # validate data type: AccountOrderUpdate
        if not isinstance(v, AccountOrderUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountOrderUpdate`")
        else:
            match += 1
        # validate data type: AccountTransactionUpdate
        if not isinstance(v, AccountTransactionUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountTransactionUpdate`")
        else:
            match += 1
        # validate data type: AccountPositionUpdate
        if not isinstance(v, AccountPositionUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountPositionUpdate`")
        else:
            match += 1
        # validate data type: AccountCommandFailureUpdate
        if not isinstance(v, AccountCommandFailureUpdate):
            error_messages.append(f"Error! Input type `{type(v)}` is not `AccountCommandFailureUpdate`")
        else:
            match += 1
        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when setting `actual_instance` in AccountStreamMessagePayload with oneOf schemas: AccountAggregatedTradeUpdate, AccountCommandFailureUpdate, AccountOrderUpdate, AccountPositionUpdate, AccountTradeUpdate, AccountTransactionUpdate, AccountUpdate. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when setting `actual_instance` in AccountStreamMessagePayload with oneOf schemas: AccountAggregatedTradeUpdate, AccountCommandFailureUpdate, AccountOrderUpdate, AccountPositionUpdate, AccountTradeUpdate, AccountTransactionUpdate, AccountUpdate. Details: " + ", ".join(error_messages))
        else:
            return v

    @classmethod
    def from_dict(cls, obj: Union[str, Dict[str, Any]]) -> Self:
        return cls.from_json(json.dumps(obj))

    @classmethod
    def from_json(cls, json_str: str) -> Self:
        """Returns the object represented by the json string"""
        instance = cls.model_construct()
        error_messages = []
        match = 0

        # deserialize data into AccountUpdate
        try:
            instance.actual_instance = AccountUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into AccountTradeUpdate
        try:
            instance.actual_instance = AccountTradeUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into AccountAggregatedTradeUpdate
        try:
            instance.actual_instance = AccountAggregatedTradeUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into AccountOrderUpdate
        try:
            instance.actual_instance = AccountOrderUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into AccountTransactionUpdate
        try:
            instance.actual_instance = AccountTransactionUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into AccountPositionUpdate
        try:
            instance.actual_instance = AccountPositionUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))
        # deserialize data into AccountCommandFailureUpdate
        try:
            instance.actual_instance = AccountCommandFailureUpdate.from_json(json_str)
            match += 1
        except (ValidationError, ValueError) as e:
            error_messages.append(str(e))

        if match > 1:
            # more than 1 match
            raise ValueError("Multiple matches found when deserializing the JSON string into AccountStreamMessagePayload with oneOf schemas: AccountAggregatedTradeUpdate, AccountCommandFailureUpdate, AccountOrderUpdate, AccountPositionUpdate, AccountTradeUpdate, AccountTransactionUpdate, AccountUpdate. Details: " + ", ".join(error_messages))
        elif match == 0:
            # no match
            raise ValueError("No match found when deserializing the JSON string into AccountStreamMessagePayload with oneOf schemas: AccountAggregatedTradeUpdate, AccountCommandFailureUpdate, AccountOrderUpdate, AccountPositionUpdate, AccountTradeUpdate, AccountTransactionUpdate, AccountUpdate. Details: " + ", ".join(error_messages))
        else:
            return instance

    def to_json(self) -> str:
        """Returns the JSON representation of the actual instance"""
        if self.actual_instance is None:
            return "null"

        if hasattr(self.actual_instance, "to_json") and callable(self.actual_instance.to_json):
            return self.actual_instance.to_json()
        else:
            return json.dumps(self.actual_instance)

    def to_dict(self) -> Optional[Union[Dict[str, Any], AccountAggregatedTradeUpdate, AccountCommandFailureUpdate, AccountOrderUpdate, AccountPositionUpdate, AccountTradeUpdate, AccountTransactionUpdate, AccountUpdate]]:
        """Returns the dict representation of the actual instance"""
        if self.actual_instance is None:
            return None

        if hasattr(self.actual_instance, "to_dict") and callable(self.actual_instance.to_dict):
            return self.actual_instance.to_dict()
        else:
            # primitive type
            return self.actual_instance

    def to_str(self) -> str:
        """Returns the string representation of the actual instance"""
        return pprint.pformat(self.model_dump())


