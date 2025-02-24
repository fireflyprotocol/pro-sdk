#!/usr/bin/env -S PYTHONPATH=../sdk/src python
#
# This script is executable, but if you prefer to specify an interpreter, please add the
# src directory to sys.path:
#
#    $ cd python
#    $ source sdk/.venv/bin/activate
#    $ PYTHONPATH=sdk/src python example/main.py

"""Sample usage of the Bluefin Pro SDK."""

import logging
import asyncio
import os
import time

from bluefin_pro_sdk import BluefinProSdk, Order, Environment, RpcUrl
from crypto_helpers.signature import SuiWallet
from openapi_client.models.account_data_stream import AccountDataStream
from openapi_client.models.transaction_type_enum import TransactionTypeEnum
import openapi_client as api

log = logging.getLogger("main")

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

LOG_LEVEL = (
    getattr(logging, level.upper())
    if (level := os.environ.get("BFP_LOG_LEVEL"))
    else logging.INFO
)

logging.basicConfig(level=LOG_LEVEL)


def now():
    """Return the current time in milliseconds since the Epoch."""
    return int(time.time() * 1000)


async def log_update(event):
    """Log an account or market data event."""
    if not type(event).__name__.endswith("Update"):
        return
    log.info(f"{event!r}")


async def main():
    """
    In this test we demonstrate how to:

    1. Log in by signing a LoginRequest payload.
    2. Get account data using a token.
    """

    # Here, we create a wallet using a mnemonic.  Alternatively, we could create a
    # wallet from private key bytes:
    #
    #   sui_wallet = SuiWallet(
    #       private_key_bytes=bytes.fromhex(
    #           "1269e3f8279bed96907a6e809a93eea2528926abbdf56584f43544859fa8c0da"
    #       )
    #   )
    sui_wallet = SuiWallet(
        private_key_bytes=bytes.fromhex(
            "3427d19dcf5781f0874c36c78aec22c03acda435d69efcbf249e8821793567a1"
        )
        # mnemonic="""
        #     dilemma salmon lake ceiling moral glide cute that ginger float area
        #     aunt vague remind cage mother concert inch dizzy present proud
        #     program time urge
        # """
    )

    # The term "sui" in the following log line is a bit redundant:  In both `sui_wallet`
    # and `sui_address`, `sui` refers to the Sui blockchain, not the SUI coin.  This is
    # unrelated to the fact that we're trading on the SUI-PERP market.
    log.info(f"{sui_wallet.sui_address=}")

    log.info(f"Connecting to {ENVIRONMENT}")
    async with BluefinProSdk(sui_wallet, contracts=None,rpc_url=RPC_URL, env=ENVIRONMENT, debug=True) as client:
        # Get Market Data from the Exchange Data API.
        exchange_data_api = client.exchange_data_api
        exchange_info = await client.exchange_data_api.get_exchange_info()
        log.info(f"{exchange_info=}")

        # We'll work with the SUI-PERP market for this example, but you could just as
        # well choose any other market available on the Bluefin Pro exchange.
        market = next(
            market for market in exchange_info.markets if market.symbol == "SUI-PERP"
        )
        log.info(f"{market=}")

        # Get a candlestick for the market.
        candlestick = await exchange_data_api.get_candlestick_data(
            market.symbol,
            api.KlineInterval.ENUM_1M,
            api.CandlePriceType.ORACLE,
        )
        log.info(f"{candlestick=}")

        # Let's get all other exchange related data to this market.  Some of these can
        # be quite large, so you may wish to truncate long lines when running this
        # sample script; for example:
        #
        #   ../example/main.py |& cut -b -160
        depth, ticker, recent_trades, funding_rate_history = await asyncio.gather(
            exchange_data_api.get_orderbook_depth(market.symbol),
            exchange_data_api.get_market_ticker(market.symbol),
            exchange_data_api.get_recent_trades(market.symbol),
            exchange_data_api.get_funding_rate_history(market.symbol),
        )
        log.info(f"{depth=} {ticker=} {recent_trades=} {funding_rate_history=}")

        # ========= Account Data API =========
        account_data_api = client.account_data_api
        when = ((t := now()) - 60 * 60 * 1000, t)  # The past hour.

        trade_history = await account_data_api.get_account_trades(market.symbol, *when)
        log.info(f"{trade_history=}")

        deposit_history = await account_data_api.get_account_transaction_history(
            [TransactionTypeEnum.DEPOSIT, TransactionTypeEnum.WITHDRAW],
            market.symbol,
            *when,
        )
        log.info(f"{deposit_history=}")

        account_details = await account_data_api.get_account_details()
        log.info(f"{account_details=}")

        account_preferences = await account_data_api.get_account_preferences()
        log.info(f"{account_preferences=}")

        # Subscribe to WebSockets and log events as they arrive.
        async with await client.create_account_data_stream_listener(
            handler=log_update
        ) as account_data_listener:
            await account_data_listener.subscribe(
                subscription=[
                    AccountDataStream.ACCOUNTORDERUPDATE,
                    AccountDataStream.ACCOUNTPOSITIONUPDATE,
                    AccountDataStream.ACCOUNTTRADEUPDATE,
                    AccountDataStream.ACCOUNTTRANSACTIONUPDATE,
                    AccountDataStream.ACCOUNTUPDATE,
                ]
            )

            async with await client.create_market_data_stream_listener(
                handler=log_update
            ) as market_data_stream:
                await market_data_stream.subscribe(
                    subscription=[
                        api.MarketSubscriptionStreams(
                            symbol=market.symbol,
                            streams=[
                                api.MarketDataStreamName.MARK_PRICE,
                                api.MarketDataStreamName.RECENT_TRADE,
                                api.MarketDataStreamName.DIFF_DEPTH_500_MS,
                                api.MarketDataStreamName.PARTIAL_DEPTH_5,
                            ],
                        )
                    ]
                )

                await asyncio.sleep(1)

                # ========= Place an Order ==========
                log.info("Creating Order")
                order_creation_result = await client.create_order(
                    Order(
                        client_order_id="123456",
                        type=api.OrderType.LIMIT,
                        symbol=market.symbol,
                        price_e9="36790000000",
                        quantity_e9="100000000000",
                        side=api.OrderSide.SHORT,
                        leverage_e9="2000000000",
                        is_isolated=False,
                        # Ten minutes hence.
                        expires_at_utc_millis=now() + 60 * 10 * 1000,
                    )
                )
                log.info(f"{order_creation_result=}")

                # ========= Get Open Orders =========
                open_orders = await client.get_open_orders(market.symbol)
                log.info(f"{open_orders=}")

                # ========== Deposit & Withdraw ==========
                # Withdraw $10.  Note that the withdraw method does its own logging.
                await client.withdraw("USDC", str(int(10e9)))

                # Listen for WebSocket events,logging them as they arrive, for a few
                # minutes before shutting down.
                await asyncio.sleep(3 * 60)


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        log.info("Exiting in response to keyboard interrupt.")
