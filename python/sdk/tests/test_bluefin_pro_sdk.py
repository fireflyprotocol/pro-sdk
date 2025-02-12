#!/usr/bin/env python

"""Integration Tests for `bluefin_pro_sdk` package."""
from pprint import pprint
import logging

import pytest

"""Integration Tests for `bluefin_pro_sdk` package."""

"""Integration Tests for `bluefin_pro_sdk` package."""

account_stream_url = "wss://stream.api.sui-staging.bluefin.io/ws/account"
market_stream_url = "wss://stream.api.sui-staging.bluefin.io/ws/market"


async def pretty_print(data):
    pprint(data)
logger = logging.getLogger("test_flow_all_services")


@pytest.mark.asyncio
async def test_flow_all_services():
    pass
