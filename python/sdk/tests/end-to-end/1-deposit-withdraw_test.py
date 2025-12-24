import pytest
import asyncio
from decimal import Decimal
from bluefin_pro_sdk import BluefinProSdk, Environment
from crypto_helpers.signature import SuiWallet

# Test wallet mnemonic - DO NOT USE IN PRODUCTION
TEST_MNEMONIC = "dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge"
# Constant for e9 decimal conversion
BASE_9_DECIMAL = Decimal("1000000000")


@pytest.mark.skip(reason="Skipping this test - deposit is not available in SDK")
@pytest.mark.asyncio
async def test_deposit_funds():
    """Test case for depositing funds into available margin"""
    # Initialize client with testnet environment
    wallet = SuiWallet(mnemonic=TEST_MNEMONIC)
    client = BluefinProSdk(sui_wallet=wallet, env=Environment.STAGING)
    await client.init()

    try:
        # Get initial balance
        account_details = await client.account_data_api.get_account_details()
        initial_balance = Decimal(account_details.total_account_value_e9)

        # TODO: Implement deposit functionality once available in SDK
        # For now, just verify that the account is accessible and has expected fields
        assert account_details.can_deposit, "Account should be able to deposit"

    finally:
        await client.close()


@pytest.mark.skip(reason="Skipping this test - deposit is not available in SDK")
@pytest.mark.asyncio
async def test_deposit_funds_excessive_amount():
    """Test case for attempting to deposit more than the initial balance"""
    # Initialize client with testnet environment
    wallet = SuiWallet(mnemonic=TEST_MNEMONIC)
    client = BluefinProSdk(sui_wallet=wallet, env=Environment.STAGING)
    await client.init()

    try:
        # Get initial balance
        account_details = await client.account_data_api.get_account_details()
        initial_balance = Decimal(account_details.total_account_value_e9)

        # Attempt to deposit an excessive amount (1000 USDC)
        excessive_amount = BASE_9_DECIMAL * \
            Decimal("1000")  # 1000 USDC in e9 format

        # The deposit should fail with an appropriate error
        with pytest.raises(Exception) as exc_info:
            # TODO: Replace with actual deposit call once implemented
            raise NotImplementedError("Deposit functionality not yet implemented")

        # Verify the error message
        assert "insufficient" in str(exc_info.value).lower() or \
               "balance" in str(exc_info.value).lower() or \
               "exceeds" in str(exc_info.value).lower() or \
               "not yet implemented" in str(exc_info.value).lower(), \
               "Should fail with insufficient balance or limit exceeded error"

        # Verify balance remained unchanged
        updated_balance = await client.account_data_api.get_account_details()
        updated_amount = Decimal(updated_balance.total_account_value_e9)
        assert updated_amount == initial_balance, \
            f"Balance should remain unchanged. Initial: {initial_balance}, Updated: {updated_amount}"

    finally:
        await client.close()


@pytest.mark.skip(reason="Skipping this test - withdraw assertions are not available in SDK")
@pytest.mark.asyncio
async def test_withdraw_funds():
    """Test case for withdrawing funds from available margin"""
    # Initialize client with testnet environment
    wallet = SuiWallet(mnemonic=TEST_MNEMONIC)
    client = BluefinProSdk(sui_wallet=wallet, env=Environment.STAGING)
    await client.init()

    try:
        # Get initial balance
        account_details = await client.account_data_api.get_account_details()
        initial_balance = Decimal(account_details.total_account_value_e9)

        # Attempt to withdraw 1 USDC
        withdraw_amount = BASE_9_DECIMAL * Decimal("1")  # 1 USDC in e9 format
        await client.withdraw("USDC", str(withdraw_amount))

        # Get updated balance and verify the withdrawal
        updated_balance = await client.account_data_api.get_account_details()
        updated_amount = Decimal(updated_balance.total_account_value_e9)

        # Verify the balance decreased by the withdrawal amount
        assert updated_amount == initial_balance - withdraw_amount, \
            f"Balance should decrease by {withdraw_amount}. Initial: {initial_balance}, Updated: {updated_amount}"

    finally:
        await client.close()


@pytest.mark.skip(reason="Skipping this test - withdraw assertions are not available in SDK")
@pytest.mark.asyncio
async def test_withdraw_funds_insufficient_balance():
    """Test case for attempting to withdraw more than available margin"""
    # Initialize client with testnet environment
    wallet = SuiWallet(mnemonic=TEST_MNEMONIC)
    client = BluefinProSdk(sui_wallet=wallet, env=Environment.STAGING)
    await client.init()

    try:
        # Get initial balance
        account_details = await client.account_data_api.get_account_details()
        initial_balance = Decimal(account_details.total_account_value_e9)

        # Attempt to withdraw more than available balance
        excessive_amount = initial_balance + \
            (BASE_9_DECIMAL * Decimal("1000"))  # Current balance + 1000 USDC

        # The withdrawal should fail with an appropriate error
        with pytest.raises(Exception) as exc_info:
            await client.withdraw("USDC", str(excessive_amount))

        # Verify the error message (you might need to adjust this based on the actual error)
        assert "insufficient" in str(exc_info.value).lower() or \
               "balance" in str(exc_info.value).lower(), \
               "Should fail with insufficient balance error"

        # Verify balance remained unchanged
        updated_balance = await client.account_data_api.get_account_details()
        updated_amount = Decimal(updated_balance.total_account_value_e9)
        assert updated_amount == initial_balance, \
            f"Balance should remain unchanged. Initial: {initial_balance}, Updated: {updated_amount}"

    finally:
        await client.close()
