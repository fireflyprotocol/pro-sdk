#!/usr/bin/env python

## Below script is combination of all three cases we manually tested for correctness
## 1. Place orders for both accounts and trade
## 2. Place orders for both accounts and trade, with some open order quantity left on the book for either account
## 3. Place orders for both accounts and trade, with some open order quantity left on the book and either account has positions on more than one market

# tests/conftest.py
import os
import pprint
from openapi_client import *
import openapi_client as api

"""Integration Tests for `bluefin_pro_sdk` package."""
import logging

# Add the src directory to sys.path


"""Integration Tests for `bluefin_pro_sdk` package."""
import asyncio
import time

from crypto_helpers.signature import SuiWallet
from openapi_client.models.account_data_stream import AccountDataStream
from openapi_client.models.trade_type_enum import TradeTypeEnum
from openapi_client.models.transaction_type_enum import TransactionTypeEnum

from bluefin_pro_sdk import BluefinProSdk, Environment, Order, RpcUrl


account_stream_url = "wss://stream.api.sui-staging.bluefin.io/ws/account"
market_stream_url = "wss://stream.api.sui-staging.bluefin.io/ws/market"

ENVIRONMENT = (
    getattr(Environment, env.upper())
    if (env := os.environ.get("BFP_ENVIRONMENT"))
    else Environment.DEV
)

RPC_URL = (
    getattr(RpcUrl, env.upper())
    if (env := os.environ.get("BFP_RPC_URL"))
    else RpcUrl.DEV
)

logger = logging.getLogger("main")

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)

# Convert e9 string to integer
def e9_to_int(e9_str):
    return int(e9_str)

# Convert integer to e9 string
def int_to_e9(value):
    return str(value)

# Define a tolerance value for comparisons
tolerance = 1e9  # Adjust this value as needed

async def place_order(client, account_number, clientId, price, size, orderType, side):
   
    # ========= Place Order ==========
    logger.info(f"Creating Order for account {account_number}")
    logger.info(f"Order Details: Account Number: {account_number}, Client Order Id: {clientId}, Price: {price}, Size: {size}, Order Type: {orderType}, Side: {side}")
    order_creation_result = await client.create_order(
        Order(client_order_id=clientId, type=orderType, symbol="ARB-PERP",
              price_e9=price, quantity_e9=size, side=side,
              leverage_e9="20000000000", is_isolated=False,
             expires_at_utc_millis = int(time.time() * 1000) + 6 * 60 * 1000))  # 5 minutes in milliseconds
    logger.info(f"Order Creation Result for account {account_number}: {order_creation_result}")


async def main():
    """Main module."""


    # Initialize wallets
    sui_wallet_1 = SuiWallet(
        mnemonic="universe hedgehog expire supply live spell hill nest someone laptop action retreat")
    sui_wallet_2 = SuiWallet(
        mnemonic="same ugly logic cherry side liquid estate valid raccoon draw flock caution")

    # Initialize clients
    async with BluefinProSdk(sui_wallet_1, contracts=None, rpc_url=RPC_URL, env=ENVIRONMENT, debug=True) as client_1, \
               BluefinProSdk(sui_wallet_2, contracts=None, rpc_url=RPC_URL, env=ENVIRONMENT, debug=True) as client_2:


        # Place orders for both accounts
        await place_order(client_1, 1, "123", "60000000000000", "5000000000", api.OrderType.LIMIT, api.OrderSide.LONG)
        await place_order(client_2, 2, "1234", "60000000000000", "10000000", api.OrderType.LIMIT, api.OrderSide.SHORT)
        

        pprint.pprint("-----------------Account 1 Details----------------")
        account_details_after_1 = await client_1.account_data_api.get_account_details()
        pprint.pprint(f"Account Details after placing orders for account 1: {account_details_after_1.to_dict()}")
        account_details_after_2 = await client_2.account_data_api.get_account_details()
        pprint.pprint("-----------------Account 2 Details----------------")
        pprint.pprint(f"Account Details after placing orders for account 2: {account_details_after_2.to_dict()}")

       

if __name__ == '__main__':
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nKeyboard interrupt received. Exiting gracefully...")