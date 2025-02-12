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
from typing import Any, ClassVar, Dict, List
from typing import Optional, Set
from typing_extensions import Self

class TickerResponse(BaseModel):
    """
    TickerResponse
    """ # noqa: E501
    symbol: StrictStr = Field(description="Market symbol.")
    last_quantity_e9: StrictStr = Field(description="Last trade quantity (e9 format).", alias="lastQuantityE9")
    last_time_at_utc_millis: StrictInt = Field(description="Last trade time in milliseconds.", alias="lastTimeAtUtcMillis")
    last_price_e9: StrictStr = Field(description="Last trade price (e9 format).", alias="lastPriceE9")
    last_funding_rate_e9: StrictStr = Field(description="Funding rate value (e9 format).", alias="lastFundingRateE9")
    next_funding_time_at_utc_millis: StrictInt = Field(description="Time in milliseconds of next funding rate update.", alias="nextFundingTimeAtUtcMillis")
    avg_funding_rate8hr_e9: StrictStr = Field(description="8 hr average funding rate (e9 format).", alias="avgFundingRate8hrE9")
    oracle_price_e9: StrictStr = Field(description="Oracle price of the asset (e9 format).", alias="oraclePriceE9")
    oracle_price_direction: StrictInt = Field(description="Direction of oracle price computed by comparing current oracle price to last oracle price. 0 = no change, -1 = negative trend (current < last), 1 positive trend (current > last).", alias="oraclePriceDirection")
    mark_price_e9: StrictStr = Field(description="Mark price on the exchange (e9 format).", alias="markPriceE9")
    mark_price_direction: StrictInt = Field(description="Direction of mark price computed by comparing current mark price to last mark price. 0 = no change, -1 = negative trend (current < last), 1 positive trend (current > last).", alias="markPriceDirection")
    market_price_e9: StrictStr = Field(description="Simple average of bestBid and bestAsk. lastPrice if either is not present (e9 format).", alias="marketPriceE9")
    market_price_direction: StrictInt = Field(description="Direction of market price computed by comparing current market price to last market price. 0 = no change, -1 = negative trend (current < last), 1 positive trend (current > last).", alias="marketPriceDirection")
    best_bid_price_e9: StrictStr = Field(description="Best bid price (e9 format).", alias="bestBidPriceE9")
    best_bid_quantity_e9: StrictStr = Field(description="Best bid quantity (e9 format).", alias="bestBidQuantityE9")
    best_ask_price_e9: StrictStr = Field(description="Best ask price (e9 format).", alias="bestAskPriceE9")
    best_ask_quantity_e9: StrictStr = Field(description="Best ask quantity (e9 format).", alias="bestAskQuantityE9")
    open_interest_e9: StrictStr = Field(description="Open interest value (e9 format).", alias="openInterestE9")
    high_price24hr_e9: StrictStr = Field(description="Highest Price in the last 24hrs (e9 format).", alias="highPrice24hrE9")
    low_price24hr_e9: StrictStr = Field(description="Lowest Price in the last 24hrs (e9 format).", alias="lowPrice24hrE9")
    volume24hr_e9: StrictStr = Field(description="Total market volume in last 24hrs of asset (e9 format).", alias="volume24hrE9")
    quote_volume24hr_e9: StrictStr = Field(description="Total market volume in last 24hrs in USDC (e9 format).", alias="quoteVolume24hrE9")
    close_price24hr_e9: StrictStr = Field(description="Close price 24hrs ago (e9 format).", alias="closePrice24hrE9")
    open_price24hr_e9: StrictStr = Field(description="Open price in the last 24hrs (e9 format).", alias="openPrice24hrE9")
    close_time24hr_at_utc_millis: StrictInt = Field(description="24 hour close time in milliseconds.", alias="closeTime24hrAtUtcMillis")
    open_time24hr_at_utc_millis: StrictInt = Field(description="24 hour open time in milliseconds.", alias="openTime24hrAtUtcMillis")
    first_id24hr: StrictInt = Field(description="First trade id in 24hr.", alias="firstId24hr")
    last_id24hr: StrictInt = Field(description="Last trade id in 24hr.", alias="lastId24hr")
    count24hr: StrictStr = Field(description="Total number of trades in 24hr.")
    price_change24hr_e9: StrictStr = Field(description="24hr Market price change (e9 format).", alias="priceChange24hrE9")
    price_change_percent24hr_e9: StrictStr = Field(description="24hr Market price change in percentage (e9 format).", alias="priceChangePercent24hrE9")
    last_updated_at_utc_millis: StrictInt = Field(description="Last update time in milliseconds.", alias="lastUpdatedAtUtcMillis")
    __properties: ClassVar[List[str]] = ["symbol", "lastQuantityE9", "lastTimeAtUtcMillis", "lastPriceE9", "lastFundingRateE9", "nextFundingTimeAtUtcMillis", "avgFundingRate8hrE9", "oraclePriceE9", "oraclePriceDirection", "markPriceE9", "markPriceDirection", "marketPriceE9", "marketPriceDirection", "bestBidPriceE9", "bestBidQuantityE9", "bestAskPriceE9", "bestAskQuantityE9", "openInterestE9", "highPrice24hrE9", "lowPrice24hrE9", "volume24hrE9", "quoteVolume24hrE9", "closePrice24hrE9", "openPrice24hrE9", "closeTime24hrAtUtcMillis", "openTime24hrAtUtcMillis", "firstId24hr", "lastId24hr", "count24hr", "priceChange24hrE9", "priceChangePercent24hrE9", "lastUpdatedAtUtcMillis"]

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
        """Create an instance of TickerResponse from a JSON string"""
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
        """Create an instance of TickerResponse from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "symbol": obj.get("symbol"),
            "lastQuantityE9": obj.get("lastQuantityE9"),
            "lastTimeAtUtcMillis": obj.get("lastTimeAtUtcMillis"),
            "lastPriceE9": obj.get("lastPriceE9"),
            "lastFundingRateE9": obj.get("lastFundingRateE9"),
            "nextFundingTimeAtUtcMillis": obj.get("nextFundingTimeAtUtcMillis"),
            "avgFundingRate8hrE9": obj.get("avgFundingRate8hrE9"),
            "oraclePriceE9": obj.get("oraclePriceE9"),
            "oraclePriceDirection": obj.get("oraclePriceDirection"),
            "markPriceE9": obj.get("markPriceE9"),
            "markPriceDirection": obj.get("markPriceDirection"),
            "marketPriceE9": obj.get("marketPriceE9"),
            "marketPriceDirection": obj.get("marketPriceDirection"),
            "bestBidPriceE9": obj.get("bestBidPriceE9"),
            "bestBidQuantityE9": obj.get("bestBidQuantityE9"),
            "bestAskPriceE9": obj.get("bestAskPriceE9"),
            "bestAskQuantityE9": obj.get("bestAskQuantityE9"),
            "openInterestE9": obj.get("openInterestE9"),
            "highPrice24hrE9": obj.get("highPrice24hrE9"),
            "lowPrice24hrE9": obj.get("lowPrice24hrE9"),
            "volume24hrE9": obj.get("volume24hrE9"),
            "quoteVolume24hrE9": obj.get("quoteVolume24hrE9"),
            "closePrice24hrE9": obj.get("closePrice24hrE9"),
            "openPrice24hrE9": obj.get("openPrice24hrE9"),
            "closeTime24hrAtUtcMillis": obj.get("closeTime24hrAtUtcMillis"),
            "openTime24hrAtUtcMillis": obj.get("openTime24hrAtUtcMillis"),
            "firstId24hr": obj.get("firstId24hr"),
            "lastId24hr": obj.get("lastId24hr"),
            "count24hr": obj.get("count24hr"),
            "priceChange24hrE9": obj.get("priceChange24hrE9"),
            "priceChangePercent24hrE9": obj.get("priceChangePercent24hrE9"),
            "lastUpdatedAtUtcMillis": obj.get("lastUpdatedAtUtcMillis")
        })
        return _obj


