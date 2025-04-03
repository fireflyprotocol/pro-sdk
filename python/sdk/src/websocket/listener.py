import asyncio
import json
import logging
from typing import List, Callable, Any, Awaitable

import websockets
from openapi_client import *

logger = logging.getLogger("data_stream_listeners")


class WebsocketEventsListener:
    def __init__(self, websocket_url: str, token: str, raw_message_handler):
        """
        Initialize the WebsocketEventsListener.

        :param websocket_url: The WebSocket URL to connect to.
        :param token: The authorization token for WebSocket connections.
        :param raw_message_handler: A coroutine function to handle incoming messages.
        """
        self.websocket_url = websocket_url
        self.token = token
        self.message_handler = raw_message_handler
        self.websocket = None
        self._listener_task = None
        self._ping_task = None
        self.connected = False

    async def connect(self):
        """Connect to the WebSocket and start listening."""
        logger.info(f"Connecting to {self.websocket_url}...")
        self.websocket = await websockets.connect(
            self.websocket_url,
            additional_headers={"Authorization": f"Bearer {self.token}"}
        )
        self._ping_task = asyncio.create_task(self._send_pings())
        self.connected = True
        logger.info("Connected to WebSocket.")

    async def close(self):
        """Close the WebSocket connection and stop listening."""
        logger.debug("Closing connection...")
        if self._listener_task:
            self._listener_task.cancel()
            try:
                await self._listener_task
            except asyncio.CancelledError:
                logger.debug("Listener task cancelled.")
        if self._ping_task:
            self._ping_task.cancel()
            try:
                await self._ping_task
            except asyncio.CancelledError:
                logger.debug("Ping task cancelled.")
        if self.websocket:
            await self.websocket.close()
            logger.debug("WebSocket closed.")
        logger.info("Connection closed...")
        self.connected = False

    async def _listen_to_websocket(self):
        """Listen for incoming WebSocket messages."""
        try:
            async for message in self.websocket:
                try:
                    # Pass the message to the handler
                    await self.message_handler(message)
                except Exception as e:
                    logger.exception(
                        f"Failed to process message: {e} payload {message}")
        except websockets.ConnectionClosed:
            logger.debug("WebSocket connection closed.")

    async def _send_pings(self):
        """Send periodic ping messages to the WebSocket."""
        try:
            while self.connected:
                await self.websocket.ping("Ping")
                logger.debug("Ping sent.")
                await asyncio.sleep(10)  # Ping interval
        except websockets.ConnectionClosed:
            logger.debug("WebSocket connection closed while sending pings.")

    async def _send_message(self, message: str):
        """
        Send a string message over the WebSocket connection.

        :param message: The message to send.
        """
        if not self.connected or not self.websocket:
            raise RuntimeError(
                "Cannot send message: WebSocket is not connected.")
        await self.websocket.send(message)

    async def __aenter__(self):
        """Enter the async context."""
        await self.connect()
        return self

    async def __aexit__(self, exc_type, exc_value, traceback):
        """Exit the async context."""
        await self.close()


class MarketDataStreamListener(WebsocketEventsListener):

    # override __init__
    def __init__(self, websocket_url: str, token: str, handler: Callable[[Any], Awaitable[None]]):
        self.__market_stream_message_handler = handler
        super().__init__(websocket_url, token, self.__handle_message)

    async def __handle_message(self, data: str):
        event_dict = json.loads(data)
        deserialize_object = None
        if "event" in event_dict.keys():
            event_type = MarketEventType(event_dict["event"])
            payload = event_dict["payload"]
            if event_type is MarketEventType.ORACLEPRICEUPDATE:
                deserialize_object = OraclePriceUpdate.from_dict(payload)
            elif event_type is MarketEventType.MARKPRICEUPDATE:
                deserialize_object = MarkPriceUpdate.from_dict(payload)
            elif event_type is MarketEventType.MARKETPRICEUPDATE:
                deserialize_object = MarketPriceUpdate.from_dict(payload)
            elif event_type is MarketEventType.CANDLESTICKUPDATE:
                deserialize_object = CandlestickUpdate.from_dict(payload)
            elif event_type is MarketEventType.ORDERBOOKDIFFDEPTHUPDATE:
                deserialize_object = OrderbookDiffDepthUpdate.model_construct(
                    **payload)
            elif event_type is MarketEventType.ORDERBOOKPARTIALDEPTHUPDATE:
                deserialize_object = OrderbookPartialDepthUpdate.from_dict(
                    payload)
            elif event_type is MarketEventType.RECENTTRADESUPDATES:
                deserialize_object = RecentTradesUpdates.from_dict(payload)
            elif event_type is MarketEventType.TICKERALLUPDATE:
                deserialize_object = TickerAllUpdate.from_dict(payload)
            elif event_type is MarketEventType.TICKERUPDATE:
                deserialize_object = TickerUpdate.from_dict(payload)
            else:
                logger.warning(
                    f"Unknown event type: {event_type} payload: {payload}")
                raise RuntimeError(
                    f"Unknown event type: {event_type} payload: {payload}")
        else:
            logger.warning(f"Unknown payload: {data}")
            raise RuntimeError(f"Unknown payload: {data}")

        await self.__market_stream_message_handler(deserialize_object)

    async def subscribe(self, subscription: List[MarketSubscriptionStreams]):
        subscription_message = MarketSubscriptionMessage.model_construct(method=SubscriptionType.SUBSCRIBE,
                                                                         data_streams=subscription).to_json()
        if not self.connected:
            raise RuntimeError("Cannot subscribe: WebSocket is not connected.")

        await self._send_message(subscription_message)

        # get subscription resopnse
        result = SubscriptionResponseMessage.from_json(await self.websocket.recv())

        if not result.success:
            raise RuntimeError(
                f"Failed to subscribe to market data streams: {result.message}")

        self._listener_task = asyncio.create_task(self._listen_to_websocket())
        logger.info(f"subscribed to streams {result.message}")


class AccountDataStreamListener(WebsocketEventsListener):

    def __init__(self, websocket_url: str, token: str, handler: Callable[[Any], Awaitable[None]]):
        self.__account_data_stream_message_handler = handler
        super().__init__(websocket_url, token, self.__handle_message)

    async def __handle_message(self, data: str):
        event_dict = json.loads(data)
        deserialize_object = None
        if "event" in event_dict.keys():
            event_type = AccountEventType(event_dict["event"])
            payload = event_dict["payload"]
            if event_type is AccountEventType.ACCOUNTUPDATE:
                deserialize_object = AccountUpdate.from_dict(payload)
            elif event_type is AccountEventType.ACCOUNTTRADEUPDATE:
                deserialize_object = AccountTradeUpdate.from_dict(payload)
            elif event_type is AccountEventType.ACCOUNTAGGREGATEDTRADEUPDATE:
                deserialize_object = AccountAggregatedTradeUpdate.from_dict(payload)
            elif event_type is AccountEventType.ACCOUNTORDERUPDATE:
                deserialize_object = AccountOrderUpdate.from_dict(payload)
            elif event_type is AccountEventType.ACCOUNTPOSITIONUPDATE:
                deserialize_object = AccountPositionUpdate.from_dict(payload)
            elif event_type is AccountEventType.ACCOUNTTRANSACTIONUPDATE:
                deserialize_object = AccountTransactionUpdate.from_dict(payload)
            else:
                logger.warning(
                    f"Unknown event type: {event_type} payload: {payload}")
                raise RuntimeError(
                    f"Unknown event type: {event_type} payload: {payload}")
        else:
            logger.warning(f"Unknown payload: {data}")
            raise RuntimeError(f"Unknown payload: {data}")

        await self.__account_data_stream_message_handler(deserialize_object)

    async def subscribe(self, subscription: List[AccountDataStream]):
        subscription_message = AccountSubscriptionMessage.model_construct(method=SubscriptionType.SUBSCRIBE,
                                                                          data_streams=subscription).to_json()

        if not self.connected:
            raise RuntimeError("Cannot subscribe: WebSocket is not connected.")

        await self._send_message(subscription_message)
        # get subscription response
        result = SubscriptionResponseMessage.from_json(await self.websocket.recv())

        if not result.success:
            raise RuntimeError(
                f"Failed to subscribe to streams: {result.message}")

        self._listener_task = asyncio.create_task(self._listen_to_websocket())
        logger.info(f"subscribed to streams {result.message}")
