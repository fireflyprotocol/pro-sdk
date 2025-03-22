import pytest
from crypto_helpers.wallet import SuiWallet
from crypto_helpers.contracts import ProContracts
from crypto_helpers.rpc import ProRpcCalls

contracts = {
    "ExternalDataStore": "0x1aba81ea1544e161c7abab342f81ff043f7abf3e8c3fa2ec59eeb26b1aa66891",
    "Package": "0x4df491e6918f6be6301ec8cc64db4469160b220555911e60b6f93578fc41bc30",
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
    mnemonic="dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge"
)

pro_contracts = ProContracts(contracts)

pro_rpc_calls = ProRpcCalls(
    sui_wallet=sui_wallet, 
    contracts=pro_contracts, 
    url="https://fullnode.testnet.sui.io:443"
)


@pytest.mark.asyncio
async def test_deposit():
    
    # USDC amount is in 6 decimal places
    # 0.15 USDC == 150000
    
    success, _ = pro_rpc_calls.deposit_to_asset_bank("USDC", 1500)

    assert success == True
    
            


