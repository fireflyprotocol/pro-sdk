import pytest
from crypto_helpers.contracts import ProContracts

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


@pytest.mark.asyncio
async def test_contract_initialization():
    
    pro_contracts = ProContracts(contracts)

    assert pro_contracts.get_external_data_store_id() == contracts["ExternalDataStore"]
    assert pro_contracts.get_internal_data_store_id() == contracts["InternalDataStore"]
    assert pro_contracts.get_package_id() == contracts["Package"]



@pytest.mark.asyncio
async def test_supported_assets():
    
    pro_contracts = ProContracts(contracts)

    token_type = pro_contracts.get_supported_token_type("USDC")

    assert token_type == "0x4df491e6918f6be6301ec8cc64db4469160b220555911e60b6f93578fc41bc30::coin::COIN"


@pytest.mark.asyncio
async def test_supported_assets_not_found():
    
    pro_contracts = ProContracts(contracts)

    with pytest.raises(Exception):
        pro_contracts.get_supported_token_type("BTC")



