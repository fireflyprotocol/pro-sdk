import pytest
from crypto_helpers.signature import SuiWallet
from openapi_client import WithdrawRequestSignedFields, CreateOrderRequestSignedFields, AccountPositionLeverageUpdateRequestSignedFields


@pytest.mark.asyncio
async def test_deposit():
    
    contracts = {
        "ExternalDataStore": "0x1aba81ea1544e161c7abab342f81ff043f7abf3e8c3fa2ec59eeb26b1aa66891",
        "AdminCap": "0x39f4027bb77db2e7e15a1e922b94959ca488783f1087ea79aeba5844439642f6",
        "Package": "0x4df491e6918f6be6301ec8cc64db4469160b220555911e60b6f93578fc41bc30",
        "UpgradeCap": "0x73750367b60ffc7f0c46d63f028d7035832873083806bb491efa0b4a600dadba",
        "TreasuryCap": "0x7a4de059c7d3bd340ab13a08059eeae9037ab6e19eba7f13b01f33654c03204a",
        "InternalDataStore": "0xcfce89f16cf7a4b54c24923fd1ed7575a093602725cfb4e7458e8b78f9b03bbc",
        "SupportedAssets": {
            "USDC": {
                "coinType": "0x4df491e6918f6be6301ec8cc64db4469160b220555911e60b6f93578fc41bc30::coin::COIN",
                "decimals": 6,
                "symbol": "USDC"
            }
        },
    }

    sui_wallet = SuiWallet(
        mnemonic="dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge",
        url="https://fullnode.testnet.sui.io:443",
        contracts=contracts
    )

    # USDC amount is in 1e6
    success, _ = sui_wallet.deposit_to_asset_bank("USDC", 1500)

    assert success == True
    
            


