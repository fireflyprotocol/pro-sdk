import unittest
from src.openapi_client import AdjustIsolatedMarginRequest, AdjustIsolatedMarginRequestSignedFields, \
    AdjustMarginOperation, OrderSide, OrderType, CreateOrderRequest, CreateOrderRequestSignedFields, \
        WithdrawRequest, WithdrawRequestSignedFields, AccountAuthorizationRequest, AccountAuthorizationRequestSignedFields, \
            AccountPositionLeverageUpdateRequest, AccountPositionLeverageUpdateRequestSignedFields
from src.crypto_helpers.hash import Hashable, PositionType


class TestHashable(unittest.TestCase):
    
    def test_withdraw_request_hash(self):
        signed_fields = WithdrawRequestSignedFields(
            eds_id="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            asset_symbol="BTC-PERP",
            account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            amount_e9="1000000000",
            salt="12345",
            signed_at_millis=1725931543867
        )
        
        request = WithdrawRequest(
            signed_fields=signed_fields,
            signature=""
        )
        
        # Create withdraw request
        hashable = Hashable.WithdrawRequest(request)
        
        # Verify fields are set correctly
        self.assertEqual(hashable.eds, signed_fields.eds_id)
        self.assertEqual(hashable.asset_symbol, signed_fields.asset_symbol)
        self.assertEqual(hashable.account, signed_fields.account_address)
        self.assertEqual(hashable.amount, int(signed_fields.amount_e9))
        self.assertEqual(hashable.salt, int(signed_fields.salt))
        self.assertEqual(hashable.signed_at, signed_fields.signed_at_millis)
        
        hash = hashable.hash()
        # Generated from the RUST SDK. The payload should give this hash.
        self.assertEqual(hash, "f637fa05edf97b1201520411dfa9656efc94e090a0e8dcdae7f24bc951ac13da")
    
    def test_authorize_account_request_hash(self): 
        # Create signed fields
        signed_fields = AccountAuthorizationRequestSignedFields(
            ids_id="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            authorized_account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            salt="67890",
            signed_at_millis=1725931543867
        )
        
        request = AccountAuthorizationRequest(
            signed_fields=signed_fields,
            signature=""
        )
        
        # Create authorize account request
        authorize_request = Hashable.AuthorizeAccountRequest(request)
        
        # Verify fields are set correctly
        self.assertEqual(authorize_request.ids, signed_fields.ids_id)
        self.assertEqual(authorize_request.account, signed_fields.account_address)
        self.assertEqual(authorize_request.user, signed_fields.authorized_account_address)
        self.assertTrue(authorize_request.status)
        self.assertEqual(authorize_request.salt, int(signed_fields.salt))
        self.assertEqual(authorize_request.signed_at, signed_fields.signed_at_millis)
        
        # Test hash method
        hash_result = authorize_request.hash()
        self.assertEqual(hash_result, "833f2bb970c74b3b2007d9be4e81386712586f4b34bde45228428a51e62113af")
    
    def test_deauthorize_account_request_hash(self):
        # Create signed fields
        signed_fields = AccountAuthorizationRequestSignedFields(
            ids_id="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            authorized_account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            salt="67890",
            signed_at_millis=1725931543867
        )
        
        request = AccountAuthorizationRequest(
            signed_fields=signed_fields,
            signature=""
        )
        
        # Create deauthorize account request
        deauthorize_request = Hashable.DeauthorizeAccountRequest(request)
        
        # Verify fields are set correctly
        self.assertEqual(deauthorize_request.ids, signed_fields.ids_id)
        self.assertEqual(deauthorize_request.account, signed_fields.account_address)
        self.assertEqual(deauthorize_request.user, signed_fields.authorized_account_address)
        self.assertFalse(deauthorize_request.status)
        self.assertEqual(deauthorize_request.salt, int(signed_fields.salt))
        self.assertEqual(deauthorize_request.signed_at, signed_fields.signed_at_millis)
        
        # Test hash method
        hash_result = deauthorize_request.hash()
        self.assertEqual(hash_result, "13b284bb260af6b7086038981d7595275589620071fe02121ed9dabc653816e6")
    
    def test_create_order_request_hash(self):
        signed_fields = CreateOrderRequestSignedFields(
            symbol="BTC-PERP",
            account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            price_e9="50000000000",
            quantity_e9="1000000000",
            side=OrderSide.LONG,
            leverage_e9="10000000000",
            is_isolated=False,
            salt="12345",
            ids_id="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            expires_at_millis=1725931543867,
            signed_at_millis=1725931543867
        )
        
        request = CreateOrderRequest(
            signed_fields=signed_fields,
            signature="",
            client_order_id="",
            type=OrderType.MARKET,
            reduce_only=False,
            post_only=False,
            time_in_force=None,
            trigger_price_e9=None,
            self_trade_prevention_type=None
        )
        
        # Create create order request
        hashable = Hashable.CreateOrderRequest(request)
        
        # Verify fields are set correctly
        self.assertEqual(hashable.ids, signed_fields.ids_id)
        self.assertEqual(hashable.account, signed_fields.account_address)
        self.assertEqual(hashable.market, signed_fields.symbol)
        self.assertEqual(hashable.price, int(signed_fields.price_e9))
        self.assertEqual(hashable.quantity, int(signed_fields.quantity_e9))
        self.assertEqual(hashable.leverage, int(signed_fields.leverage_e9))
        self.assertEqual(hashable.side, signed_fields.side.value)
        self.assertEqual(hashable.position_type, 
                         PositionType.ISOLATED.value if signed_fields.is_isolated else PositionType.CROSS.value)
        self.assertEqual(hashable.expiration, signed_fields.expires_at_millis)
        self.assertEqual(hashable.salt, int(signed_fields.salt))
        self.assertEqual(hashable.signed_at, signed_fields.signed_at_millis)
        
        # Test hash method
        hash_result = hashable.hash()
        self.assertEqual(hash_result, "c372f185f8ba230f4144102b7d4ce0a8840d17516696fdbe0ca2a135e147370e")


    def test_adjust_leverage_request_hash(self):
        signed_fields = AccountPositionLeverageUpdateRequestSignedFields(
            ids_id="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            symbol="BTC-PERP",
            leverage_e9="10000000000",
            salt="12345",
            signed_at_millis=1725931543867
        )
        
        request = AccountPositionLeverageUpdateRequest(
            signed_fields=signed_fields,
            signature=""
        )
        
        # Create adjust leverage request
        hashable = Hashable.AdjustLeverageRequest(request)
        
        # Verify fields are set correctly
        self.assertEqual(hashable.ids, signed_fields.ids_id)
        self.assertEqual(hashable.account, signed_fields.account_address)
        self.assertEqual(hashable.market, signed_fields.symbol)
        self.assertEqual(hashable.leverage, int(signed_fields.leverage_e9))
        self.assertEqual(hashable.salt, int(signed_fields.salt))
        self.assertEqual(hashable.signed_at, signed_fields.signed_at_millis)
        
        # Test hash method
        hash_result = hashable.hash()
        self.assertEqual(hash_result, "e4200a4f1a8d977ba86d5a7e1218499bd4a1b9ee47d42cc91096236833d74d69")
    
    def test_adjust_isolated_margin_request_hash(self):
        signed_fields = AdjustIsolatedMarginRequestSignedFields(
            ids_id="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            account_address="0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786",
            symbol="BTC-PERP",
            operation=AdjustMarginOperation.ADD,
            quantity_e9="1000000000",
            salt="12345",
            signed_at_millis=1725931543867
        )
        
        request = AdjustIsolatedMarginRequest(
            signedFields=signed_fields,
            signature=""
        )
        
        # Create adjust isolated margin request
        hashable = Hashable.AdjustIsolatedMarginRequest(request)
        
        # Verify fields are set correctly
        self.assertEqual(hashable.ids, signed_fields.ids_id)
        self.assertEqual(hashable.account, signed_fields.account_address)
        self.assertEqual(hashable.market, signed_fields.symbol)
        self.assertEqual(hashable.add, signed_fields.operation == AdjustMarginOperation.ADD)
        self.assertEqual(hashable.amount, int(signed_fields.quantity_e9))
        self.assertEqual(hashable.salt, int(signed_fields.salt))
        self.assertEqual(hashable.signed_at, signed_fields.signed_at_millis)

        # Test hash method
        hash_result = hashable.hash()
        self.assertEqual(hash_result, "8f12ca81965e16955d523103a440eca598e9b3155c431ce8ef1d7a4268c48f43")
    
    
if __name__ == '__main__':
    unittest.main()
