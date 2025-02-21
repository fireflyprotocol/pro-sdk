import json
import base64
from hashlib import blake2b
from openapi_client import WithdrawRequestSignedFields, \
    AccountPositionLeverageUpdateRequestSignedFields, \
    CreateOrderRequestSignedFields
from openapi_client.models.login_request import LoginRequest

from crypto_helpers.bcs import BCSSerializer
from crypto_helpers.wallet import SuiWallet

class Signature:

    def __init__(self, sui_wallet: SuiWallet):
        self.sui_wallet = sui_wallet

    def login(self, payload: LoginRequest) -> bytes:
        """
        Creates a withdrawal request message and signs it

        Args:
            payload: withdraw payload.

        Returns:
            base 64 encoded signature bytes

        """
        data = {
            "type": "Bluefin Pro Login",
            "accountAddress": payload.account_address,
            "signedAtUtcMillis": str(payload.signed_at_utc_millis),
            "audience": payload.audience,
        }

        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key    


    def withdraw(self, payload: WithdrawRequestSignedFields) -> str:
        """
        Creates a withdrawal request message and signs it

        Args:
            payload: withdraw payload.

        Returns:
            base 64 encoded signature bytes

        """
        data = {
            "type": "Bluefin Pro Withdrawal",
            "eds": payload.eds_id,
            "assetSymbol": payload.asset_symbol,
            "account": payload.account_address,
            "amount": str(payload.amount_e9),
            "salt": str(payload.salt),
            "signedAt": str(payload.signed_at_utc_millis)
        }


        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key    

    def order(self, payload: CreateOrderRequestSignedFields) -> bytes:
        """
        Creates an order and signs it

        Args:
            payload: order payload.

        Returns:
            base 64 encoded signature bytes

        """
        data = {
            "type": "Bluefin Pro Order",
            "ids": payload.ids_id,
            "account": payload.account_address,
            "market": payload.symbol,
            "price": str(payload.price_e9),
            "quantity": str(payload.quantity_e9),
            "leverage": str(payload.leverage_e9),
            "side": payload.side,
            "positionType": "ISOLATED" if payload.is_isolated == True else "CROSS",
            "expiration": str(payload.expires_at_utc_millis),
            "salt": str(payload.salt),
            "signedAt": str(payload.signed_at_utc_millis)
        }

        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key

    def adjust_leverage(self, payload: AccountPositionLeverageUpdateRequestSignedFields) -> bytes:
        """
        Signs adjust leverage request

        Args:
            payload: adjust leverage payload.

        Returns:
            Signature bytes

        """
        data = {
            "type": "Bluefin Pro Leverage Adjustment",
            "ids": payload.ids_id,
            "account": payload.account_address,
            "market": payload.symbol,
            "leverage": str(payload.leverage_e9),
            "salt": str(payload.salt),
            "signedAt": str(payload.signed_at_utc_millis)
        }

        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key
    
    def adjust_margin(self, payload) -> bytes:
        """
        Signs adjust margin request

        Args:
            payload: adjust margin payload.

        Returns:
            base 64 encoded signature bytes


        """
        data = {
            "type": "Bluefin Pro Margin Adjustment",
            "ids": payload["ids_id"],
            "account": payload["account_address"],
            "market": payload["symbol"],
            "add": payload["add"],
            "amount": payload["amount_e9"],
            "salt": str(payload["salt"]),
            "signedAt": str(payload["signed_at_utc_millis"])
        }

        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key
    
    def authorize_account(self, payload) -> bytes:
        """
        Signs an account authorization request

        Args:
            payload: authorize account payload.

        Returns:
            base 64 encoded signature bytes


        """
        data = {
            "type": "Bluefin Pro Authorize Account",
            "ids": payload["ids_id"],
            "account": payload["account_address"],
            "user": payload["user_address"],
            "status": payload["status"],
            "salt": str(payload["salt"]),
            "signedAt": str(payload["signed_at_utc_millis"])
        }

        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key
    
    def close_position(self, payload) -> bytes:
        """
        Signs close position request

        Args:
            payload: close position payload.

        Returns:
            base 64 encoded signature bytes


        """
        data = {
            "type": "Bluefin Pro Close Position For Delisted Market",
            "ids": payload["ids_id"],
            "account": payload["account_address"],
            "market": payload["symbol"],
            "isolated": payload["is_isolated"],
            "salt": str(payload["salt"]),
            "signedAt": str(payload["signed_at_utc_millis"])
        }


        message = self.create_personal_sign_message(data)

        signature = self.sign(message)

        base64_signature_with_public_key = self.build_base_64_signature_with_scheme_and_public_key(signature)

        return base64_signature_with_public_key

    def create_personal_sign_message(self, data:json) -> bytes:
        """
        Python implementation of `signPersonalMessage()` method from mysten/sui package.
        Blue pro verifies signatures against personal sign message. The method converts the input
        data into the personal sign message and returns it:
        1. JSON stringify the data and convert to bytes
        2. BCS serialize the data bytes
        3. Append personal message intent bytes
        4. Take blake2b hash of the above message with intent bytes
    
        Args:
            data (json): A JSON payload to be signed

        Returns:
            A personal sign message ready to be signed
        """
        
        serializer = BCSSerializer()

        # Json stringify with indent and encode to ut8 bytes
        uint8_array = list(json.dumps(data, indent=2).encode("utf-8")) 

        # bcs serialize 
        serializer.serialize_uint8_array(uint8_array)

        # serialized bytes
        serialized_bytes = list(serializer.get_bytes())

        # append intent bytes
        message_with_intent = [3,0,0] + serialized_bytes

        # 32 bytes hash
        message = blake2b(bytes(message_with_intent), digest_size=32).digest()

        return message
    
    def create_hash(self, data:json) -> bytes:
        """
        Creates the data 
        1. JSON stringify the data and convert to bytes
        2. BCS serialize the data bytes
        3. Append personal message intent bytes
        4. Take blake2b hash of the above message with intent bytes
    
        Args:
            data (json): A JSON payload to be signed

        Returns:
            A personal sign message ready to be signed
        """
        
        serializer = BCSSerializer()

        # Json stringify with indent and encode to ut8 bytes
        uint8_array = list(json.dumps(data, indent=2).encode("utf-8")) 

        # bcs serialize 
        serializer.serialize_uint8_array(uint8_array)

        # serialized bytes
        serialized_bytes = list(serializer.get_bytes())

        # append intent bytes
        message_with_intent = [3,0,0] + serialized_bytes

        # 32 bytes hash
        message = blake2b(bytes(message_with_intent), digest_size=32).digest()

        return message

    def sign(self, message: bytes) -> bytes:
        """
        Signs the given bytes of message and returns the signature
    
        Args:
            message (bytes): message to be signed

        Returns:
            generated signature
        """

        signature = self.sui_wallet.private_key.sign(bytearray(message))

        return signature
    
    def build_base_64_signature_with_scheme_and_public_key(self, signature: bytes) -> str:
        """
        Builds a base64 signature of 97/98 bytes containing:
        - 1st Bytes of wallet scheme
        - 2nd to 65th byte of signature
        - 66 to 97/98 byte containing public key
    
        Args:
            signature (bytes): 64 byte signature

        Returns:
            Base 64 encoded string signature
        """

        # the scheme is always zero as the constructor of the SuiWallet 
        # class decodes incoming wallet phrase as ED25519
        # NOTE: please recover wallet scheme and use that over here
        byte_array = [0] + list(signature) + list(self.sui_wallet.public_key_bytes)


        return base64.b64encode(bytes(byte_array))



