import time

import pytest
from bluefin_api import WithdrawRequestSignedFields

from crypto_helpers.signature import SuiWallet


@pytest.mark.asyncio
async def test_withdraw_signature():
    sui_wallet = SuiWallet(
        mnemonic="dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge")
    print("Sui Address: ", sui_wallet.sui_address)

    withdraw_payload = WithdrawRequestSignedFields(
        asset_symbol='USDC',
        asset_bank_address='0x32bdb7183ea037cd99aa49f7e3fe19d846543a53b9d92c33e750f313ccfc3d63',
        account_address='0x17f48479d2b749382e87eedd3413dafb3f47c5520e7fd79c1067441396ebed37',
        amount_e9="5000000000",
        salt="1736456558023",
        signed_at_utc_millis=int(time.time() * 1000),
    )

    expected_serialization = "045553444332bdb7183ea037cd99aa49f7e3fe19d846543a53b9d92c33e750f313ccfc3d6317f48479d2b749382e87eedd3413dafb3f47c5520e7fd79c1067441396ebed3700f2052a01000000c755e04c940100004c0ff14c94010000"
    expected_hash = "8b947e0b5d31b399aa22fd359a838c95f0f85e6af529fd764386dbbeebd79499"
    expected_signature = "c0bc7593f7b44b58698df74a597fc86e163f450908032f51a60026d9f50a02830cc3158c74674d5bd966b0d94451674bdc2a139065d223f83beffc9974355805"
    expected_serialized_signature = "40c0bc7593f7b44b58698df74a597fc86e163f450908032f51a60026d9f50a02830cc3158c74674d5bd966b0d94451674bdc2a139065d223f83beffc9974355805201a2e5082447bc0fefa8dea75be25e8fdcdf72353a41093bc3a7741542b5be5c200"

    [serialized_payload, hash, signature,
        serialized_signature] = sui_wallet.sign_withdraw_request(withdraw_payload)

    print('\nBCS Serialized Payload:', serialized_payload.hex())

    print("\nSha256 Hash: ", hash.hex())

    print("\nSignature: ", signature.hex())

    print("\nSerialized Signature: ", serialized_signature.hex())

    print("\nChecks:")
    print("-> Serialization: {}".format(serialized_payload.hex()
          == expected_serialization))
    print("-> Hash: {}".format(hash.hex() == expected_hash))
    print("-> Signature: {}".format(signature.hex() == expected_signature))
    print("-> Serialized Signature: {}".format(serialized_signature.hex()
          == expected_serialized_signature))
