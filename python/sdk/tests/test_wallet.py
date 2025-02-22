import pytest
from crypto_helpers.wallet import SuiWallet


mnemonic="dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge"

@pytest.mark.asyncio
async def test_wallet_initialization():
    
    sui_wallet = SuiWallet(mnemonic=mnemonic)
    assert sui_wallet.sui_address == "0x17f48479d2b749382e87eedd3413dafb3f47c5520e7fd79c1067441396ebed37"

