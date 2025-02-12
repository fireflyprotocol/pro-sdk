import base64
from hashlib import blake2b, sha256

from bip_utils import Bip44, Bip44Coins, Bip39SeedGenerator, Bip39MnemonicValidator, Bip39Languages
from openapi_client import LoginRequest, WithdrawRequestSignedFields, \
    AccountPositionLeverageUpdateRequestSignedFields, CreateOrderRequestSignedFields
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

from crypto_helpers.bcs import BCSSerializer


class DepositRequest:
    pass


class SuiWallet:
    def __init__(self, private_key_bytes: bytes = None, mnemonic: str = None):
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

    def sign_order_request(self, signed_fields: CreateOrderRequestSignedFields) -> (str, str):
        serializer = BCSSerializer()

        serializer.serialize_str(signed_fields.symbol)
        serializer.serialize_address(signed_fields.account_address)
        serializer.serialize_u64(int(signed_fields.price_e9))
        serializer.serialize_u64(int(signed_fields.quantity_e9))
        serializer.serialize_u64(int(signed_fields.leverage_e9))
        serializer.serialize_str(signed_fields.side.value)
        serializer.serialize_bool(signed_fields.is_isolated)
        serializer.serialize_u64(signed_fields.expires_at_utc_millis)
        serializer.serialize_u64(int(signed_fields.salt))
        serializer.serialize_address(signed_fields.ids_id)
        serializer.serialize_u64(signed_fields.signed_at_utc_millis)

        bcs_serialized_bytes = serializer.get_bytes()

        [hash_result, _signature, serialized_signature] = self.sign_payload(
            bcs_serialized_bytes)

        # only serialized signature is required to be sent back as this is what is sent to api-gateway
        return hash_result.hex(), serialized_signature.hex()

    def sign_withdraw_request(self, signed_fields: WithdrawRequestSignedFields) -> (str, str):
        serializer = BCSSerializer()

        serializer.serialize_str(signed_fields.asset_symbol)
        serializer.serialize_address(signed_fields.account_address)
        serializer.serialize_u64(int(signed_fields.amount_e9))
        serializer.serialize_u64(int(signed_fields.salt))
        serializer.serialize_address(signed_fields.eds_id)
        serializer.serialize_u64(signed_fields.signed_at_utc_millis)

        bcs_serialized_bytes = serializer.get_bytes()

        [hash_result, _signature, serialized_signature] = self.sign_payload(
            bcs_serialized_bytes)

        # only serialized signature is required to be sent back as this is what is sent to api-gateway
        return hash_result.hex(), serialized_signature.hex()

    def sign_account_position_leverage_update_request(self,
                                                      signed_fields: AccountPositionLeverageUpdateRequestSignedFields) -> (str, str):
        serializer = BCSSerializer()

        serializer.serialize_address(signed_fields.account_address)
        serializer.serialize_str(signed_fields.symbol)
        serializer.serialize_u64(int(signed_fields.leverage_e9))
        serializer.serialize_u64(int(signed_fields.salt))
        serializer.serialize_address(signed_fields.ids_id)
        serializer.serialize_u64(signed_fields.signed_at_utc_millis)

        bcs_serialized_bytes = serializer.get_bytes()

        [hash_result, _signature, serialized_signature] = self.sign_payload(
            bcs_serialized_bytes)

        return hash_result.hex(), serialized_signature.hex()

    def deposit_onchain_call(self, deposit_request: DepositRequest):
        # todo: yameen
        pass

    def sign_payload(self, serialized_bytes: bytearray):

        hash_result = sha256(serialized_bytes).digest()

        signature = self.private_key.sign(hash_result)

        payload = {
            'signature': signature,
            'public_key': self.public_key_bytes,
            'scheme': 0
        }

        serializer = BCSSerializer()
        serializer.serialize_uint8_array(list(bytearray(payload["signature"])))
        serializer.serialize_uint8_array(
            list(bytearray(payload["public_key"])))
        serializer.serialize_u8(payload["scheme"])

        return hash_result, payload["signature"], serializer.get_bytes()
