import asyncio
import logging
import time
from dataclasses import dataclass
from enum import Enum
from os import environ
from random import randint
from typing import Callable, Awaitable, Any

from openapi_client import OrderType, OrderTimeInForce, SelfTradePreventionType, OrderSide
from openapi_client import WithdrawRequestSignedFields, CancelOrdersRequest, \
    AccountPositionLeverageUpdateRequestSignedFields, CreateOrderRequestSignedFields, CreateOrderRequest
from openapi_client.api.account_data_api import AccountDataApi
from openapi_client.api.auth_api import AuthApi
from openapi_client.api.exchange_api import ExchangeApi
from openapi_client.api.trade_api import TradeApi
from openapi_client.api_client import ApiClient
from openapi_client.configuration import Configuration
from openapi_client.models.account_position_leverage_update_request import AccountPositionLeverageUpdateRequest
from openapi_client.models.login_request import LoginRequest
from openapi_client.models.withdraw_request import WithdrawRequest

from crypto_helpers.wallet import SuiWallet
from crypto_helpers.signature import Signature
from crypto_helpers.contracts import ProContracts
from crypto_helpers.rpc import ProRpcCalls

from websocket.listener import MarketDataStreamListener, AccountDataStreamListener


# enum Environment with sui-prod sui-staging sui-dev
class Environment(str, Enum):
    PRODUCTION = 'sui-prod'
    STAGING = 'sui-staging'
    DEV = 'sui-dev'


class RpcUrl(str, Enum):
    DEV = 'https://fullnode.devnet.sui.io:443'


@dataclass
class Order:
    client_order_id: str
    type: OrderType
    symbol: str
    price_e9: str
    quantity_e9: str
    side: OrderSide
    leverage_e9: str
    is_isolated: bool
    expires_at_utc_millis: int
    reduce_only: bool = False
    post_only: bool = False
    time_in_force: OrderTimeInForce = OrderTimeInForce.GTT
    trigger_price_e9: str = None
    self_trade_prevention_type: SelfTradePreventionType = None


logger = logging.getLogger(__name__)


def generate_salt():
    return str(int(time.time() * 1000) + randint(1, 1000000))


class BluefinProSdk:
    _sui_wallet = SuiWallet
    current_account_address = None
    _token_response = None
    _token_set_at_seconds = None
    __is_connected = False
    __update_token_task = None
    # todo set to dynamic value
    __contracts_config = None

    def __init__(self, 
                 sui_wallet: SuiWallet, 
                 contracts: ProContracts, 
                 rpc_url: str,
                 env: Environment = Environment.PRODUCTION, 
                 authorized_address: str = None,
                 debug: bool = False
                 ):
        """
        :param sui_wallet: SuiWallet instance
        :param env: Environment enum, default is Environment.PRODUCTION
        :param authorized_address: default is None, specify If you want to act on behalf of another accountAddress (if that user authorized you to do so)
        """
        env_name = env.value
        self._sui_wallet = sui_wallet
        self.auth_host = f"https://auth.api.{env_name}.bluefin.io"
        self.api_host = f"https://api.{env_name}.bluefin.io"
        self.trade_host = f"https://trade.api.{env_name}.bluefin.io"
        self.account_data_stream_url = f"wss://stream.api.{env_name}.bluefin.io/ws/account"
        self.market_data_stream_url = f"wss://stream.api.{env_name}.bluefin.io/ws/market"

        self.current_account_address = authorized_address
        auth_api_config = Configuration(
            host=self.auth_host, debug=debug)
        self._auth_api = AuthApi(ApiClient(auth_api_config))

        exchange_api_config = Configuration(
            host=self.api_host, debug=debug)
        self.exchange_data_api = ExchangeApi(ApiClient(exchange_api_config))

        account_data_api_config = Configuration(
            host=self.api_host, debug=debug)
        self.account_data_api = AccountDataApi(
            ApiClient(account_data_api_config))

        trade_api_config = Configuration(
            host=self.trade_host, debug=debug)
        self._trade_api = TradeApi(ApiClient(trade_api_config))

        # on-chain stuff
        self.contracts = contracts
        self._sui_wallet = sui_wallet
        self.sign = Signature(sui_wallet)
        self.rpc_calls = ProRpcCalls(sui_wallet, contracts, url=rpc_url)

    async def init(self):
        await self.__login_and_update_token()
        self.__update_token_task = asyncio.create_task(self.__refresh_token())
        self.__is_connected = True
        self.__contracts_config = (await self.exchange_data_api.get_exchange_info()).contracts_config

    async def __login_and_update_token(self):
        await self._login()
        self.account_data_api.api_client.set_default_header("Authorization",
                                                            "Bearer " + self._token_response.access_token)
        self._trade_api.api_client.set_default_header("Authorization",
                                                      "Bearer " + self._token_response.access_token)

    async def get_open_orders(self, market_address: str):
        await self._set_access_token(self._trade_api.api_client)
        return await self._trade_api.get_open_orders(market_address)

    async def update_leverage(self, symbol: str, leverage_e9: str):

        signed_fields = AccountPositionLeverageUpdateRequestSignedFields(
            account_address=self.current_account_address,
            symbol=symbol,
            leverage_e9=leverage_e9,
            salt=generate_salt(),
            signed_at_utc_millis=int(time.time() * 1000),
            ids_id=self.__contracts_config.ids_id
        )

        signature = self.sign.adjust_leverage(signed_fields)

        return await self._trade_api.put_leverage_update(
            AccountPositionLeverageUpdateRequest(
                signed_fields=signed_fields, signature=signature, request_hash="")
        )

    async def create_order(self, order: Order):
        signed_fields = CreateOrderRequestSignedFields(symbol=order.symbol,
                                                       ids_id=self.__contracts_config.ids_id,
                                                       account_address=self.current_account_address,
                                                       price_e9=order.price_e9,
                                                       quantity_e9=order.quantity_e9, side=order.side,
                                                       leverage_e9=order.leverage_e9, is_isolated=order.is_isolated,
                                                       salt=generate_salt(),
                                                       expires_at_utc_millis=order.expires_at_utc_millis,
                                                       signed_at_utc_millis=int(time.time() * 1000))

        signature = self.sign.order(signed_fields)

        create_order_request = CreateOrderRequest(
            signed_fields=signed_fields,
            signature=signature,
            order_hash="",
            client_order_id=order.client_order_id,
            type=order.type,
            reduce_only=order.reduce_only,
            post_only=order.post_only,
            time_in_force=order.time_in_force,
            trigger_price_e9=order.trigger_price_e9,
            self_trade_prevention_type=order.self_trade_prevention_type,
        )
        logger.debug(
            f"Send order creation payload {create_order_request.to_json()}")
        return await self._trade_api.post_create_order(create_order_request)

    async def cancel_order(self, cancel_orders_request: CancelOrdersRequest):
        return await self._trade_api.cancel_orders(cancel_orders_request)

    async def withdraw(self, asset_symbol: str, amount_e9: str):
        signed_fields = WithdrawRequestSignedFields(
            asset_symbol=asset_symbol,
            # for now account_address is always sui_address
            account_address=self.current_account_address,
            amount_e9=amount_e9,
            salt=generate_salt(),
            signed_at_utc_millis=int(time.time() * 1000),
            eds_id=self.__contracts_config.eds_id,
        )

        signature = self.sign.withdraw(signed_fields)

        request = WithdrawRequest(
            signed_fields=signed_fields, signature=signature, request_hash="")

        await self._trade_api.post_withdraw(request)
        logger.info(f"Withdraw request sent successfully {request}")


    async def _set_access_token(self, api_client: ApiClient):
        """
        This method is used to set the access token in the api client while using the cached access token.
        :param api_client:
        :return:
        """
        await self._login()
        api_client.set_default_header("Authorization",
                                      "Bearer " + self._token_response.access_token)

    async def _login(self):
        logging.info("Logging in to get the access token")
        self._token_set_at_seconds = time.time()
        if self.current_account_address is None:
            # in case of cross account authorisation
            self.current_account_address = self._sui_wallet.sui_address

        logger.info(f"Logging in as {self.current_account_address}")
        login_request = LoginRequest(
            account_address=self.current_account_address,
            signed_at_utc_millis=int(time.time() * 1000),
            audience="api"
        )
        # Generate a signature for the login request with our private key and public key bytes.
        signature = self.sign.login(login_request)
        response = await self._auth_api.auth_token_post(signature, login_request=login_request)
        self._token_response = response

    async def __aenter__(self):
        await self.init()  # Ensure connection is established before entering the context
        return self

    async def __aexit__(self, exc_type, exc_value, traceback):
        await self.close()  # Ensure the connection is closed when exiting the context

    async def get_access_token(self):
        if not self.__is_connected:
            raise RuntimeError("Not connected")
        return self._token_response.access_token

    async def create_account_data_stream_listener(self,
                                                  handler: Callable[[Any], Awaitable[None]]):
        return AccountDataStreamListener(self.account_data_stream_url,
                                         self._token_response.access_token, handler)

    async def create_market_data_stream_listener(self,
                                                 handler: Callable[[Any], Awaitable[None]]) -> MarketDataStreamListener:
        return MarketDataStreamListener(self.market_data_stream_url,
                                        self._token_response.access_token, handler)

    async def close(self):
        logger.info("Gracefully Closing")

        if self.__update_token_task:
            self.__update_token_task.cancel()  # Cancel the listener task
            try:
                await self.__update_token_task
            except asyncio.CancelledError:
                logger.debug("Update Token task cancelled.")

        await self._trade_api.api_client.close()
        await self.account_data_api.api_client.close()
        await self.exchange_data_api.api_client.close()
        await self._auth_api.api_client.close()

    async def __refresh_token(self):
        while True:
            logger.debug("check token for refresh")
            # Check validity of tokens every make sure it's actually using refresh tokens or just remove refresh token response
            if time.time() - self._token_set_at_seconds > self._token_response.access_token_valid_for_seconds:
                logger.debug("Refreshing token")
                # todo refresh token instead
                self._token_set_at_seconds = time.time()
                await self.__login_and_update_token()
            await asyncio.sleep(10)
