import json
import unittest
from openapi_client.models.recent_trades_updates import RecentTradesUpdates
from openapi_client.models.account_trade_update import AccountTradeUpdate
from openapi_client.models.trade import Trade


class TestWebsocketDeserialization(unittest.TestCase):
    def test_account_trade_update_deserialization(self):
       
        # Sample JSON data for AccountTradeUpdate as a string
        json_str = '''
        {
            "event": "AccountTradeUpdate",
            "reason": "OrderMatched",
            "payload": {
                "trade": {
                    "id": "43027",
                    "clientOrderId": "1743788884425",
                    "symbol": "ETH-PERP",
                    "orderHash": "5012079388d718467a08faba6219445f08b286c3c81d785215a2bb435809e15a",
                    "tradeType": "ORDER",
                    "side": "LONG",
                    "isMaker": true,
                    "priceE9": "1822300000000",
                    "quantityE9": "160000000",
                    "quoteQuantityE9": "291568000000",
                    "positionSide": "LONG",
                    "tradingFeeE9": "43735200",
                    "tradingFeeAsset": "USDC",
                    "gasFeeE9": "30000000",
                    "gasFeeAsset": "USDC",
                    "executedAtMillis": 1743788900544
                }
            }
        }
        '''
        
        # Parse the JSON string
        event_dict = json.loads(json_str)
        payload = event_dict["payload"]
        
        # Create AccountTradeUpdate object from the payload
        account_trade_update = AccountTradeUpdate.from_dict(payload)
        
        # Verify the deserialized object
        self.assertIsNotNone(account_trade_update)
        self.assertIsNotNone(account_trade_update.trade)
        
        # Verify trade properties
        trade = account_trade_update.trade
        self.assertEqual(trade.id, "43027")
        self.assertEqual(trade.client_order_id, "1743788884425")
        self.assertEqual(trade.symbol, "ETH-PERP")
        self.assertEqual(trade.order_hash, "5012079388d718467a08faba6219445f08b286c3c81d785215a2bb435809e15a")
        self.assertEqual(trade.trade_type, "ORDER")
        self.assertEqual(trade.side, "LONG")
        self.assertTrue(trade.is_maker)
        self.assertEqual(trade.price_e9, "1822300000000")
        self.assertEqual(trade.quantity_e9, "160000000")
        self.assertEqual(trade.quote_quantity_e9, "291568000000")
        self.assertEqual(trade.position_side, "LONG")
        self.assertEqual(trade.trading_fee_e9, "43735200")
        self.assertEqual(trade.trading_fee_asset, "USDC")
        self.assertEqual(trade.gas_fee_e9, "30000000")
        self.assertEqual(trade.gas_fee_asset, "USDC")
        self.assertEqual(trade.executed_at_millis, 1743788900544)

    def test_recent_trades_updates_deserialization(self):
        # Sample JSON data for RecentTradesUpdates
        json_str = '''
        {
            "event": "RecentTradesUpdates",
            "payload": {
                "trades": [
                    {
                        "id": "43023",
                        "symbol": "ETH-PERP",
                        "side": "LONG",
                        "priceE9": "1822320000000",
                        "quantityE9": "70000000",
                        "quoteQuantityE9": "127562400000",
                        "executedAtMillis": 1743788900541
                    },
                    {
                        "id": "43024",
                        "symbol": "ETH-PERP",
                        "side": "LONG",
                        "priceE9": "1822310000000",
                        "quantityE9": "60000000",
                        "quoteQuantityE9": "109338600000",
                        "executedAtMillis": 1743788900541
                    },
                    {
                        "id": "43025",
                        "symbol": "ETH-PERP",
                        "side": "LONG",
                        "priceE9": "1822310000000",
                        "quantityE9": "120000000",
                        "quoteQuantityE9": "218677200000",
                        "executedAtMillis": 1743788900541
                    },
                    {
                        "id": "43026",
                        "symbol": "ETH-PERP",
                        "side": "LONG",
                        "priceE9": "1822300000000",
                        "quantityE9": "30000000",
                        "quoteQuantityE9": "54669000000",
                        "executedAtMillis": 1743788900541
                    },
                    {
                        "id": "43027",
                        "symbol": "ETH-PERP",
                        "side": "LONG",
                        "priceE9": "1822300000000",
                        "quantityE9": "160000000",
                        "quoteQuantityE9": "291568000000",
                        "executedAtMillis": 1743788900544
                    }
                ]
            }
        }
        '''

        # Parse the JSON string
        event_dict = json.loads(json_str)
        payload = event_dict["payload"]
        
        # Deserialize the payload into a RecentTradesUpdates object
        recent_trades_updates = RecentTradesUpdates.from_dict(payload)
        
        # Verify the deserialization was successful
        self.assertIsNotNone(recent_trades_updates)
        self.assertEqual(len(recent_trades_updates.trades), 5)
        
        # Verify the first trade's properties
        first_trade = recent_trades_updates.trades[0]
        self.assertEqual(first_trade.id, "43023")
        self.assertEqual(first_trade.symbol, "ETH-PERP")
        self.assertEqual(first_trade.side, "LONG")
        self.assertEqual(first_trade.price_e9, "1822320000000")
        self.assertEqual(first_trade.quantity_e9, "70000000")
        self.assertEqual(first_trade.quote_quantity_e9, "127562400000")
        self.assertEqual(first_trade.executed_at_millis, 1743788900541)
        
        # Verify the last trade's properties
        last_trade = recent_trades_updates.trades[4]
        self.assertEqual(last_trade.id, "43027")
        self.assertEqual(last_trade.executed_at_millis, 1743788900544)


    async def test_account_trade_update_handle_message(self):
        """Test that the AccountDataStreamListener can handle an AccountTradeUpdate message."""
        # Create a JSON string representing an AccountTradeUpdate event
        json_str = '''
        {
            "event": "AccountTradeUpdate",
            "reason": "OrderMatched",
            "payload": {
                "trade": {
                    "id": "43027",
                    "clientOrderId": "1743788884425",
                    "symbol": "ETH-PERP",
                    "orderHash": "5012079388d718467a08faba6219445f08b286c3c81d785215a2bb435809e15a",
                    "tradeType": "ORDER",
                    "side": "LONG",
                    "isMaker": true,
                    "priceE9": "1822300000000",
                    "quantityE9": "160000000",
                    "quoteQuantityE9": "291568000000",
                    "positionSide": "LONG",
                    "tradingFeeE9": "43735200",
                    "tradingFeeAsset": "USDC",
                    "gasFeeE9": "30000000",
                    "gasFeeAsset": "USDC",
                    "executedAtMillis": 1743788900544
                }
            }
        }
        '''
        
        # Create a mock handler that will store the received message
        received_message = None
        
        async def mock_handler(message):
            nonlocal received_message
            received_message = message
        
        # Create an AccountDataStreamListener with the mock handler
        listener = AccountDataStreamListener("wss://example.com", "fake_token", mock_handler)
        
        # Call the private __handle_message method directly
        await listener._AccountDataStreamListener__handle_message(json_str)
        
        # Verify the message was properly deserialized and passed to the handler
        self.assertIsNotNone(received_message)
        self.assertIsInstance(received_message, AccountTradeUpdate)
        
        # Verify the trade properties
        trade = received_message.trade
        self.assertEqual(trade.id, "43027")
        self.assertEqual(trade.client_order_id, "1743788884425")
        self.assertEqual(trade.symbol, "ETH-PERP")
        self.assertEqual(trade.order_hash, "5012079388d718467a08faba6219445f08b286c3c81d785215a2bb435809e15a")
        self.assertEqual(trade.trade_type, "ORDER")
        self.assertEqual(trade.side, "LONG")
        self.assertTrue(trade.is_maker)
        self.assertEqual(trade.price_e9, "1822300000000")
        self.assertEqual(trade.quantity_e9, "160000000")
        self.assertEqual(trade.quote_quantity_e9, "291568000000")
        self.assertEqual(trade.position_side, "LONG")
        self.assertEqual(trade.trading_fee_e9, "43735200")
        self.assertEqual(trade.trading_fee_asset, "USDC")
        self.assertEqual(trade.gas_fee_e9, "30000000")
        self.assertEqual(trade.gas_fee_asset, "USDC")
        self.assertEqual(trade.executed_at_millis, 1743788900544)
