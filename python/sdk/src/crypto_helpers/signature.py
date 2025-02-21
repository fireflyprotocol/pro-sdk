import base64
from hashlib import blake2b, sha256
from typing import Any, Dict

from bip_utils import Bip44, Bip44Coins, Bip39SeedGenerator, Bip39MnemonicValidator, Bip39Languages
from crypto_helpers.rpc import get_coin_having_balance, rpc_unsafe_moveCall, rpc_sui_executeTransactionBlock
from openapi_client import LoginRequest, WithdrawRequestSignedFields, \
    AccountPositionLeverageUpdateRequestSignedFields, CreateOrderRequestSignedFields
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from crypto_helpers.bcs import BCSSerializer
import json
import base64

class SuiWallet:

    def __init__(self, private_key_bytes: bytes = None, mnemonic: str = None, url: str = None, contracts: Dict[str, Any] = None):
        if private_key_bytes is None and mnemonic is None:
            raise ValueError(
                "Either private_key_bytes or mnemonic must be provided")

        if private_key_bytes is not None:
            self.private_key = Ed25519PrivateKey.from_private_bytes(
                private_key_bytes)
        else:
            self.private_key = self.__private_key_from_mnemonic(mnemonic)

        self.public_key_bytes = self.private_key.public_key().public_bytes(
            encoding=serialization.Encoding.Raw,
            format=serialization.PublicFormat.Raw
        )
        # sui addresses are public key suffixed with the type of the pair we only support Ed25519PrivateKey for now
        public_key_typed = bytearray(1 + len(self.public_key_bytes))
        public_key_typed[0] = 0
        public_key_typed[1:1 + len(self.public_key_bytes)
                         ] = self.public_key_bytes
        hash_digest = blake2b(public_key_typed, digest_size=32).digest()
        # Convert to a hexadecimal string
        self.sui_address = "0x" + hash_digest.hex()

        self.url = url
        self.contracts = contracts



    @staticmethod
    def __private_key_from_mnemonic(mnemonic: str) -> Ed25519PrivateKey:
        """
        Derive an Ed25519 private key from a BIP-39 mnemonic.

        :param mnemonic: The BIP-39 mnemonic phrase.
        :return: Ed25519PrivateKey object.
        """
        # Validate the mnemonic
        if not Bip39MnemonicValidator(Bip39Languages.ENGLISH).IsValid(mnemonic):
            raise ValueError("Invalid mnemonic phrase Bip39Languages.ENGLISH")

        # Generate seed from the mnemonic
        seed_bytes = Bip39SeedGenerator(mnemonic).Generate()

        # Derive the Ed25519 private key using BIP-44 (or directly from the seed)
        bip44_wallet = Bip44.FromSeed(seed_bytes, Bip44Coins.SUI)
        derived_private_key_bytes = bip44_wallet.DeriveDefaultPath().PrivateKey().Raw().ToBytes()

        # Convert the derived private key bytes into an Ed25519PrivateKey
        return Ed25519PrivateKey.from_private_bytes(derived_private_key_bytes)

    def generate_login_request_signature(self, login_request: LoginRequest) -> str:
        """
        Sign the login request using the account's private key.

        Args:
            login_request: The login request to sign.

        Returns:
            A tuple containing the signed login request and the signature.
        """
        # Sign the login request
        login_request_json = login_request.to_json().encode("utf-8")
        digest = blake2b(login_request_json, digest_size=32).digest()
        signature_bytes = self.private_key.sign(digest)

        # Serialize the signature: [SigFlag (1 byte)] + [Signature] + [Public Key]
        sig_flag_ed25519 = 0  # Replace this with the appropriate SigFlag value if needed
        serialized_signature = bytearray(
            1 + len(signature_bytes) + len(self.public_key_bytes))
        serialized_signature[0] = sig_flag_ed25519
        serialized_signature[1:1 + len(signature_bytes)] = signature_bytes
        serialized_signature[1 + len(signature_bytes):] = self.public_key_bytes

        # Encode serialized signature to Base64 (URL-safe)
        return base64.urlsafe_b64encode(serialized_signature).decode('utf-8')

    def sign_withdraw_request(self, payload: WithdrawRequestSignedFields) -> bytes:
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
    

    def sign_order(self, payload: CreateOrderRequestSignedFields) -> bytes:
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


    def sign_adjust_leverage(self, payload: AccountPositionLeverageUpdateRequestSignedFields) -> bytes:
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
    
    def sign_adjust_margin(self, payload) -> bytes:
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
    
    def sign_authorize_account(self, payload) -> bytes:
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
    
    def sign_close_position(self, payload) -> bytes:
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

        # hash the bytes
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

        signature = self.private_key.sign(bytearray(message))

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
        byte_array = [0] + list(signature) + list(self.public_key_bytes)


        return base64.b64encode(bytes(byte_array))


    def sign_tx(self, tx_bytes: str) -> str:
        """
        Signs the provided transaction bytes and returns the signature

        Args:
            tx_bytes (str): The transaction bytes to be signed

        Returns:
            generated signature

        """
        tx_bytes = base64.b64decode(tx_bytes)

        intent = bytearray()
        intent.extend([0, 0, 0])
        intent = intent + tx_bytes
        hash = blake2b(intent, digest_size=32).digest()

        result = self.private_key.sign(hash)
        temp = bytearray()
        temp.append(0)
        temp.extend(result)
        temp.extend(self.public_key_bytes)
        res = base64.b64encode(temp)
        return res.decode()
    
    def deposit_to_asset_bank(self, coin_symbol: str, amount: int, destination: str = None):
        """
        Deposits the provided coin of provided amount
        into the external asset bank
    
        Args:

            coin_symbol (str): The symbol of the coin being deposited
            amount (uint): The amount to be deposited
            destination (str): Optional destination account to which funds are being deposited.
                               By default, funds are always deposited to the depositor's account

        """

        coin_details = self.contracts["SupportedAssets"][coin_symbol]
        coin_type = coin_details["coinType"]

        # get the coin for deposit
        coin_id = get_coin_having_balance(
            user_address=self.sui_address,
            coin_type=coin_type,
            balance=amount,
            url=self.url,
            exact_match=False
        )


        # prepare the transaction arguments
        move_function_params = [
                    self.contracts["ExternalDataStore"],
                    coin_symbol,
                    self.sui_address if destination == None else destination,
                    str(amount),
                    coin_id
                ]
                
        tx_bytes = rpc_unsafe_moveCall(
            url=self.url,
            params=move_function_params,
            function_name='deposit_to_asset_bank',
            function_library='exchange',
            userAddress=self.sui_address,
            packageId=self.contracts["Package"],
            gasBudget=10000000,
            typeArguments=[ coin_type ]
        )

        signature = self.sign_tx(tx_bytes)
        res = rpc_sui_executeTransactionBlock(self.url, tx_bytes, signature)
        try:
            success = res["result"]["effects"]["status"]["status"] == "success"
            return success, res
        except Exception as e:
            return False , res
        


