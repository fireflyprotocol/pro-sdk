import asyncio
import logging
import time
from dataclasses import dataclass
from enum import Enum
from random import randint
from typing import Callable, Awaitable, Any
from typing_extensions import deprecated

from openapi_client import OrderType, OrderTimeInForce, SelfTradePreventionType, OrderSide
from openapi_client import WithdrawRequestSignedFields, CancelOrdersRequest, \
    AccountPositionLeverageUpdateRequestSignedFields, CreateOrderRequestSignedFields, CreateOrderRequest, AccountAuthorizationRequest, AccountAuthorizationRequestSignedFields, AdjustIsolatedMarginRequest, AdjustIsolatedMarginRequestSignedFields, AdjustMarginOperation
from openapi_client.api.account_data_api import AccountDataApi
from openapi_client.api.auth_api import AuthApi
from openapi_client.api.exchange_api import ExchangeApi
from openapi_client.api.rewards_api import RewardsApi
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
from crypto_helpers.hash import Hashable as BluefinHashable
from websocket.listener import MarketDataStreamListener, AccountDataStreamListener


# enum Environment with environment name and RPC URL
class Environment(str, Enum):
    PRODUCTION = ('sui-prod', 'https://fullnode.mainnet.sui.io:443')
    STAGING = ('sui-staging', 'https://fullnode.testnet.sui.io:443')
    DEV = ('sui-dev', 'https://fullnode.testnet.sui.io:443')

    def __new__(cls, env_name: str, rpc_url: str):
        obj = str.__new__(cls, env_name)
        obj._value_ = env_name
        obj._rpc_url = rpc_url
        return obj

    @property
    def env_name(self) -> str:
        return self.value

    @property
    def rpc_url(self) -> str:
        return self._rpc_url

class AccountAuthorizationAction(Enum):
    AUTHORIZE = True
    DEAUTHORIZE = False

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
    expires_at_millis: int
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
    __rpc_calls = None
    _read_only = False
    _refresh_token_valid_for_seconds = None
    def __init__(self,
                 sui_wallet: SuiWallet,
                 env: Environment = Environment.PRODUCTION,
                 target_account_address: str = None,
                 debug: bool = False,
                 colocation_enabled: bool = False,
                 read_only: bool = False,
                 refresh_token_valid_for_seconds: int = None,
                 ):
        """
        :param sui_wallet: SuiWallet instance
        :param env: Environment enum, default is Environment.PRODUCTION
        :param target_account_address: default is None if target account belongs to the same sui wallet, if you need to act on behalf of another sui wallet on its own accountAddress (if that wallet authorized you to do so)
        :param read_only: default is False, if True, the sdk will be in read-only mode, allowing only read access to the API
        :param refresh_token_valid_for_seconds: default is 30 days if not provided. If provided, the refresh token will be valid for the given number of seconds
        """
        self.env = env
        env_name = env.env_name
        self._sui_wallet = sui_wallet
        self.auth_host = f"https://auth.api.{env_name}.bluefin.io"
        self.api_host = f"https://api.{env_name}.bluefin.io"
        self.sign = Signature(sui_wallet)
        self.__contracts_config = None
        self._read_only = read_only
        self._refresh_token_valid_for_seconds = refresh_token_valid_for_seconds

        if colocation_enabled:
            # Through AWS private link traffic is already secured and we don't need encryption.
            self.trade_host = f"http://api.coloc.{env_name}.int.bluefin.io:9090"
            self.account_data_stream_url = f"ws://api.coloc.{env_name}.int.bluefin.io:9091/ws/account"
            self.market_data_stream_url = f"ws://api.coloc.{env_name}.int.bluefin.io:9091/ws/market"
        else:
            self.trade_host = f"https://trade.api.{env_name}.bluefin.io"
            self.account_data_stream_url = f"wss://stream.api.{env_name}.bluefin.io/ws/account"
            self.market_data_stream_url = f"wss://stream.api.{env_name}.bluefin.io/ws/market"

        self.current_account_address = target_account_address
        auth_api_config = Configuration(
            host=self.auth_host, debug=debug)
        self._auth_api = AuthApi(ApiClient(auth_api_config))

        exchange_api_config = Configuration(
            host=self.api_host, debug=debug)
        self.exchange_data_api = ExchangeApi(ApiClient(exchange_api_config))

        rewards_api_config = Configuration(
            host=self.api_host, debug=debug)
        self.rewards_data_api = RewardsApi(ApiClient(rewards_api_config))

        account_data_api_config = Configuration(
            host=self.api_host, debug=debug)
        self.account_data_api = AccountDataApi(
            ApiClient(account_data_api_config))

        trade_api_config = Configuration(
            host=self.trade_host, debug=debug)
        self._trade_api = TradeApi(ApiClient(trade_api_config))

        # on-chain stuff

    async def init(self):
        await self.__login_and_update_token()
        self.__update_token_task = asyncio.create_task(self.__refresh_token())
        self.__is_connected = True
        # set contracts and rpc calls
        exchange_info = await self.exchange_data_api.get_exchange_info()
        self.__contracts_config = exchange_info.contracts_config
        # todo: dynamic supoorted assets once we support multi collat
        contracts_config = {
            "ExternalDataStore": exchange_info.contracts_config.eds_id,
            "InternalDataStore": exchange_info.contracts_config.ids_id,
            "Package": exchange_info.contracts_config.current_contract_address,
            "Operators": exchange_info.contracts_config.operators,
            "SupportedAssets": {
                "USDC": {
                    "coinType": exchange_info.assets[0].asset_type,
                    "decimals": exchange_info.assets[0].decimals,
                    "symbol": exchange_info.assets[0].symbol
                }
            }
        }
        # set RpcUrl enum from
        self.__rpc_calls = ProRpcCalls(self._sui_wallet, ProContracts(contracts_config), url=self.env.rpc_url)

    def deposit_to_asset_bank(self, asset_symbol: str, amount_e9: str, destination_address: str = None):
        """
        Deposits the provided asset of provided amount into the external asset bank.
        :param asset_symbol: The symbol of the asset being deposited (e.g. "USDC")
        :param amount_e9: The amount to be deposited in 9 decimal places (e.g. 1 USDC == 1000000)
        :param destination_address: Optional destination account address to which funds are being deposited.
                                    By default, funds are always deposited to the depositor's account
        """
        if not self.__is_connected:
            raise RuntimeError("Not connected. Please call init() first.")
        if destination_address is None:
            destination_address = self.current_account_address or self._sui_wallet.sui_address

        self.__rpc_calls.deposit_to_asset_bank(asset_symbol, amount_e9, destination_address)

    async def __login_and_update_token(self):
        await self._login()
        self.account_data_api.api_client.set_default_header("Authorization",
                                                            "Bearer " + self._token_response.access_token)
        self._trade_api.api_client.set_default_header("Authorization",
                                                      "Bearer " + self._token_response.access_token)
        self.rewards_data_api.api_client.set_default_header("Authorization",
                                                            "Bearer " + self._token_response.access_token)


    def compute_hash(self, request: CreateOrderRequest | AdjustIsolatedMarginRequest | AccountAuthorizationRequest | AccountPositionLeverageUpdateRequest | WithdrawRequest):
        """
        Computes the hash for various request types.

        Args:
            request: The request object to hash (CreateOrderRequest, AdjustIsolatedMarginRequest,
                    AccountAuthorizationRequest, AccountPositionLeverageUpdateRequest, or WithdrawRequest)

        Returns:
            str: The computed hash as a hexadecimal string
        """

        if isinstance(request, CreateOrderRequest):
            hashable = BluefinHashable.CreateOrderRequest(request)
        elif isinstance(request, AdjustIsolatedMarginRequest):
            hashable = BluefinHashable.AdjustIsolatedMarginRequest(request)
        elif isinstance(request, AccountAuthorizationRequest):
            hashable = BluefinHashable.AuthorizeAccountRequest(request)
        elif isinstance(request, AccountPositionLeverageUpdateRequest):
            hashable = BluefinHashable.AdjustLeverageRequest(request)
        elif isinstance(request, WithdrawRequest):
            hashable = BluefinHashable.WithdrawRequest(request)

        return hashable.hash()

    async def get_open_orders(self, symbol: str):
        await self._set_access_token(self._trade_api.api_client)
        return await self._trade_api.get_open_orders(symbol)

    async def get_standby_orders(self, symbol: str):
        await self._set_access_token(self._trade_api.api_client)
        return await self._trade_api.get_standby_orders(symbol)

    async def update_leverage(self, symbol: str, leverage_e9: str):
        signed_fields = AccountPositionLeverageUpdateRequestSignedFields(
            account_address=self.current_account_address,
            symbol=symbol,
            leverage_e9=leverage_e9,
            salt=generate_salt(),
            signed_at_millis=int(time.time() * 1000),
            ids_id=self.__contracts_config.ids_id
        )

        signature = self.sign.adjust_leverage(signed_fields)

        return await self._trade_api.put_leverage_update(
            AccountPositionLeverageUpdateRequest(
                signed_fields=signed_fields, signature=signature, request_hash="")
        )

    async def adjust_isolated_margin(self, symbol: str, quantity_e9: str, add: bool):
        signed_fields = AdjustIsolatedMarginRequestSignedFields(
            account_address=self.current_account_address,
            symbol=symbol,
            quantity_e9=quantity_e9,
            operation=AdjustMarginOperation.ADD if add else AdjustMarginOperation.SUBTRACT,
            salt=generate_salt(),
            signed_at_millis=int(time.time() * 1000),
            ids_id=self.__contracts_config.ids_id
        )

        signature = self.sign.adjust_isolated_margin(signed_fields)

        return await self._trade_api.put_adjust_isolated_margin(
            AdjustIsolatedMarginRequest(
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
                                                       expires_at_millis=order.expires_at_millis,
                                                       signed_at_millis=int(time.time() * 1000))

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

    async def cancel_standby_order(self, cancel_orders_request: CancelOrdersRequest):
        return await self._trade_api.cancel_standby_orders(cancel_orders_request)

    async def withdraw(self, asset_symbol: str, amount_e9: str):
        signed_fields = WithdrawRequestSignedFields(
            asset_symbol=asset_symbol,
            # for now account_address is always sui_address
            account_address=self.current_account_address,
            amount_e9=amount_e9,
            salt=generate_salt(),
            signed_at_millis=int(time.time() * 1000),
            eds_id=self.__contracts_config.eds_id,
        )

        signature = self.sign.withdraw(signed_fields)

        request = WithdrawRequest(
            signed_fields=signed_fields, signature=signature, request_hash="")

        await self._trade_api.post_withdraw(request)
        logger.info(f"Withdraw request sent successfully {request}")

    async def authorize_account(self, authorized_account_address: str):
        signed_fields = AccountAuthorizationRequestSignedFields(
            account_address=self.current_account_address,
            authorized_account_address=authorized_account_address,
            ids_id=self.__contracts_config.ids_id,
            salt=generate_salt(),
            signed_at_millis=int(time.time() * 1000),
        )

        signature = self.sign.authorize_account(signed_fields, is_authorize=AccountAuthorizationAction.AUTHORIZE.value)

        await self._trade_api.put_authorize_account(
            AccountAuthorizationRequest(
                signed_fields=signed_fields, signature=signature, request_hash="")
        )

        logger.info(f"Authorize account request sent successfully {signed_fields}")

    async def deauthorize_account(self, authorized_account_address: str):
        signed_fields = AccountAuthorizationRequestSignedFields(
            account_address=self.current_account_address,
            authorized_account_address=authorized_account_address,
            ids_id=self.__contracts_config.ids_id,
            salt=generate_salt(),
            signed_at_millis=int(time.time() * 1000),
        )

        signature = self.sign.authorize_account(signed_fields, is_authorize=AccountAuthorizationAction.DEAUTHORIZE.value)

        await self._trade_api.put_deauthorize_account(
            AccountAuthorizationRequest(
                signed_fields=signed_fields, signature=signature, request_hash="")
        )

        logger.info(f"Deauthorize account request sent successfully {signed_fields}")

    async def _set_access_token(self, api_client: ApiClient):
        """
        This method is used to set the access token in the api client while using the cached access token.
        :param api_client:
        :return:
        """
        await self._login()
        api_client.set_default_header("Authorization",
                                      "Bearer " + self._token_response.access_token)

    async def _login(self, v1: bool = True, **kwargs):
        
        kwargs['refresh_token_valid_for_seconds'] = self._refresh_token_valid_for_seconds
        kwargs['read_only'] = self._read_only
        
        if v1:
            await self._login_v1(**kwargs)
        else:
            await self._login_v2(**kwargs)
    
    async def _login_v1(self, **kwargs):
        """
        login v1 method with optional parameters.
        
        Args:
            **kwargs: Optional parameters that can be extended in the future
                - refresh_token_valid_for_seconds: Optional int for refresh token validity
                - read_only: Optional bool for read-only token
        """
        
        logging.info("Logging in to get the access token")
        self._token_set_at_seconds = time.time()
        if self.current_account_address is None:
            # in case of cross account authorisation
            self.current_account_address = self._sui_wallet.sui_address

        logger.info(f"Logging in as {self.current_account_address}")
        login_request = LoginRequest(
            account_address=self.current_account_address,
            signed_at_millis=int(time.time() * 1000),
            audience="api"
        )
        # Generate a signature for the login request with our private key and public key bytes.
        signature = self.sign.login(login_request)
        
        refresh_token_valid_for_seconds = kwargs.get('refresh_token_valid_for_seconds')
        read_only = kwargs.get('read_only')
        
        response = await self._auth_api.auth_token_post(
            signature, 
            login_request=login_request,
            refresh_token_valid_for_seconds=refresh_token_valid_for_seconds,
            read_only=read_only
        )
        self._token_response = response
        print(f"Token response V1: {self._token_response}")
        
    async def _login_v2(self, **kwargs):
        """
        Enhanced login v2 method with optional parameters.
        
        Args:
            **kwargs: Optional parameters that can be extended in the future
                - refresh_token_valid_for_seconds: Optional int for refresh token validity
                - read_only: Optional bool for read-only token
        """
        
        logging.info("Logging in to get the access token")
        self._token_set_at_seconds = time.time()
        if self.current_account_address is None:
            # in case of cross account authorisation
            self.current_account_address = self._sui_wallet.sui_address

        logger.info(f"Logging in as {self.current_account_address}")
        login_request = LoginRequest(
            account_address=self.current_account_address,
            signed_at_millis=int(time.time() * 1000),
            audience="api"
        )
        # Generate a signature for the login request with our private key and public key bytes.
        signature = self.sign.login_v2(login_request)
        refresh_token_valid_for_seconds = kwargs.get('refresh_token_valid_for_seconds')
        read_only = kwargs.get('read_only')
        
        response = await self._auth_api.auth_v2_token_post(
            signature, 
            login_request=login_request, 
            refresh_token_valid_for_seconds=refresh_token_valid_for_seconds, 
            read_only=read_only
        )
        self._token_response = response
        print(f"Token response V2: {self._token_response}")
        

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
        await self.rewards_data_api.api_client.close()
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
