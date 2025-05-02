from enum import Enum
from openapi_client.models.create_order_request import CreateOrderRequest
from openapi_client.models.account_authorization_request import AccountAuthorizationRequest
from openapi_client.models.withdraw_request import WithdrawRequest
from openapi_client.models.account_position_leverage_update_request import AccountPositionLeverageUpdateRequest
from openapi_client.models.adjust_isolated_margin_request import AdjustIsolatedMarginRequest
from openapi_client.models.adjust_margin_operation import AdjustMarginOperation
from crypto_helpers.bcs import BCSSerializer
from hashlib import blake2b

class PositionType(str, Enum):
    ISOLATED = "ISOLATED"
    CROSS = "CROSS"

class Hashable:
    """
    Contains classes for creating hashable representations of API requests.
    """
    
    class WithdrawRequest:
        def __init__(self, request: WithdrawRequest):
            self.eds = request.signed_fields.eds_id
            self.asset_symbol = request.signed_fields.asset_symbol
            self.account = request.signed_fields.account_address
            self.amount = int(request.signed_fields.amount_e9)
            self.salt = int(request.signed_fields.salt)
            self.signed_at = int(request.signed_fields.signed_at_millis)
        
            
        def hash(self):
            serializer = BCSSerializer()

            serializer.serialize_address(self.eds)
            serializer.serialize_str(self.asset_symbol)
            serializer.serialize_address(self.account)
            serializer.serialize_u64(self.amount)
            serializer.serialize_u64(self.salt)
            serializer.serialize_u64(self.signed_at)
            
            bcs_bytes = serializer.get_bytes()
            hex_encoded = bcs_bytes.hex()
            
            print(hex_encoded)
            
            return blake2b(hex_encoded.encode(), digest_size=32).hexdigest()
        
    
    class AuthorizeAccountRequest:
        def __init__(self, request: AccountAuthorizationRequest):
            self.ids = request.signed_fields.ids_id
            self.account = request.signed_fields.account_address
            self.user = request.signed_fields.authorized_account_address
            self.status = True
            self.salt = int(request.signed_fields.salt)
            self.signed_at = int(request.signed_fields.signed_at_millis)
        
        def hash(self):
            serializer = BCSSerializer()
            serializer.serialize_address(self.ids)
            serializer.serialize_address(self.account)
            serializer.serialize_address(self.user)
            serializer.serialize_bool(self.status)
            serializer.serialize_u64(self.salt)
            serializer.serialize_u64(self.signed_at)
            
            # Get serialized bytes and convert to hex
            bcs_bytes = serializer.get_bytes()
            hex_encoded = bcs_bytes.hex()
            
            # Use Blake2b hash on the hex-encoded bytes
            return blake2b(hex_encoded.encode(), digest_size=32).hexdigest()
            
    class DeauthorizeAccountRequest:
        def __init__(self, request: AccountAuthorizationRequest):
            self.ids = request.signed_fields.ids_id
            self.account = request.signed_fields.account_address
            self.user = request.signed_fields.authorized_account_address
            self.status = False
            self.salt = int(request.signed_fields.salt)
            self.signed_at = int(request.signed_fields.signed_at_millis)
        
        def hash(self):
            serializer = BCSSerializer()
            
            serializer.serialize_address(self.ids)
            serializer.serialize_address(self.account)
            serializer.serialize_address(self.user)
            serializer.serialize_bool(self.status)
            serializer.serialize_u64(self.salt)
            serializer.serialize_u64(self.signed_at)
            
            # Get serialized bytes and convert to hex
            bcs_bytes = serializer.get_bytes()
            hex_encoded = bcs_bytes.hex()
            
            # Use Blake2b hash on the hex-encoded bytes
            return blake2b(hex_encoded.encode(), digest_size=32).hexdigest()
    
    class CreateOrderRequest:
        def __init__(self, request: CreateOrderRequest):
            self.ids = request.signed_fields.ids_id
            self.account = request.signed_fields.account_address
            self.market = request.signed_fields.symbol
            self.price = int(request.signed_fields.price_e9)
            self.quantity = int(request.signed_fields.quantity_e9)
            self.leverage = int(request.signed_fields.leverage_e9)
            self.side = request.signed_fields.side.value
            self.position_type = PositionType.ISOLATED.value if request.signed_fields.is_isolated else PositionType.CROSS.value
            self.expiration = int(request.signed_fields.expires_at_millis)
            self.salt = int(request.signed_fields.salt)
            self.signed_at = int(request.signed_fields.signed_at_millis)
            
        def hash(self):
            serializer = BCSSerializer()
            serializer.serialize_address(self.ids)
            serializer.serialize_address(self.account)
            serializer.serialize_str(self.market)
            serializer.serialize_u64(self.price)
            serializer.serialize_u64(self.quantity)
            serializer.serialize_u64(self.leverage)
            serializer.serialize_str(self.side)
            serializer.serialize_str(self.position_type)
            serializer.serialize_u64(self.expiration)
            serializer.serialize_u64(self.salt)
            serializer.serialize_u64(self.signed_at)
            
            # Get serialized bytes and convert to hex
            bcs_bytes = serializer.get_bytes()
            hex_encoded = bcs_bytes.hex()
            
            # Use Blake2b hash on the hex-encoded bytes
            return blake2b(hex_encoded.encode(), digest_size=32).hexdigest()
    
    class AdjustLeverageRequest:
        def __init__(self, request: AccountPositionLeverageUpdateRequest):
            self.ids = request.signed_fields.ids_id
            self.account = request.signed_fields.account_address
            self.market = request.signed_fields.symbol
            self.leverage = int(request.signed_fields.leverage_e9)
            self.salt = int(request.signed_fields.salt)
            self.signed_at = int(request.signed_fields.signed_at_millis)
        
        def hash(self):
            serializer = BCSSerializer()
            serializer.serialize_address(self.ids)
            serializer.serialize_address(self.account)
            serializer.serialize_str(self.market)
            serializer.serialize_u64(self.leverage)
            serializer.serialize_u64(self.salt)
            serializer.serialize_u64(self.signed_at)
            
            # Get serialized bytes and convert to hex
            bcs_bytes = serializer.get_bytes()
            hex_encoded = bcs_bytes.hex()
            
            # Use Blake2b hash on the hex-encoded bytes
            return blake2b(hex_encoded.encode(), digest_size=32).hexdigest()
    
    class AdjustIsolatedMarginRequest:
        def __init__(self, request: AdjustIsolatedMarginRequest):            
            self.ids = request.signed_fields.ids_id
            self.account = request.signed_fields.account_address
            self.market = request.signed_fields.symbol
            self.add = request.signed_fields.operation == AdjustMarginOperation.ADD
            self.amount = int(request.signed_fields.quantity_e9)
            self.salt = int(request.signed_fields.salt)
            self.signed_at = int(request.signed_fields.signed_at_millis)
            
        def hash(self):
            serializer = BCSSerializer()
            serializer.serialize_address(self.ids)
            serializer.serialize_address(self.account)
            serializer.serialize_str(self.market)
            serializer.serialize_bool(self.add)
            serializer.serialize_u64(self.amount)
            serializer.serialize_u64(self.salt)
            serializer.serialize_u64(self.signed_at)
            
            # Get serialized bytes and convert to hex
            bcs_bytes = serializer.get_bytes()
            hex_encoded = bcs_bytes.hex()
            
            # Use Blake2b hash on the hex-encoded bytes
            return blake2b(hex_encoded.encode(), digest_size=32).hexdigest()
    

