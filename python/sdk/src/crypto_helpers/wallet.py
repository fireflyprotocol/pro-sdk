from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from bip_utils import Bip44, Bip44Coins, Bip39SeedGenerator, Bip39MnemonicValidator, Bip39Languages

from hashlib import blake2b
from typing import Optional

class SuiWallet:

    def __init__(self, private_key_bytes: Optional[bytes] = None, mnemonic: Optional[str] = None):
        if private_key_bytes is None and mnemonic is None:
            raise ValueError(
                "Either private_key_bytes or mnemonic must be provided")

        if private_key_bytes is not None:
            self.private_key = Ed25519PrivateKey.from_private_bytes(
                private_key_bytes)
        elif mnemonic is not None:
            self.private_key = self.__private_key_from_mnemonic(mnemonic)
        else:
            raise ValueError("Either private_key_bytes or mnemonic must be provided")

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

