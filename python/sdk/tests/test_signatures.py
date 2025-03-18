import pytest
from crypto_helpers.signature import SuiWallet, Signature
from openapi_client import WithdrawRequestSignedFields, CreateOrderRequestSignedFields, AccountPositionLeverageUpdateRequestSignedFields, AccountAuthorizationRequestSignedFields

sui_wallet = SuiWallet(
    mnemonic="dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge"
)

sign = Signature(sui_wallet)

@pytest.mark.asyncio
async def test_withdraw_signature():

    payload = WithdrawRequestSignedFields(
        asset_symbol='USDC',
        eds_id='0x5ffe1778e69001cb87f457ac3997df7567d54c4010b8193e6797793213f29ea0',
        account_address='0x76a831f3a961579e5fd34b6cda412409dd41ba18fe0f10e149ecdad413af4050',
        amount_e9="1500000",
        salt="1739231818963",
        signed_at_millis=1739233815612,
    )

    signature = sign.withdraw(payload)

    # the expected signature is created using TS SDK
    assert signature == "AJ+zoZuwbEvwk/xEZneanZPifervwLCLTHZEh9/WlxJpSrbMtxjfY6phwCpIc7PF2VQ/TozEH74RiZN8xJo+IgkaLlCCRHvA/vqN6nW+Jej9zfcjU6QQk7w6d0FUK1vlwg=="


@pytest.mark.asyncio
async def test_order_signature():

    payload = CreateOrderRequestSignedFields(
        symbol="ETH-PERP",
        account_address="0x76a831f3a961579e5fd34b6cda412409dd41ba18fe0f10e149ecdad413af4050",
        price_e9="100000000000",
        quantity_e9="100000000",
        side="LONG",
        leverage_e9="0",
        is_isolated=False,
        salt= "1739491998372",
        ids_id="0x38a9b1c542212bccc84bdf315afda5a15f00662520c25b31b47cdf87a3705ac9",
        expires_at_millis= 2037603360000,
        signed_at_millis=1739492801736,
    )

    signature = sign.order(payload)

    # the expected signature is created using TS SDK
    assert signature == "AC/fjarm+Mx0/Hl8qFlkuheTSvBU2xlR5gX3up8+5ZvtDngZitFuDwK20RMtVLF6F1zd8oPZKdtHklMIkjbVugoaLlCCRHvA/vqN6nW+Jej9zfcjU6QQk7w6d0FUK1vlwg=="



@pytest.mark.asyncio
async def test_adjust_leverage_signature():

    payload = AccountPositionLeverageUpdateRequestSignedFields(
        ids_id="0x38a9b1c542212bccc84bdf315afda5a15f00662520c25b31b47cdf87a3705ac9",
        account_address="0x76a831f3a961579e5fd34b6cda412409dd41ba18fe0f10e149ecdad413af4050",
        symbol="ETH-PERP",
        leverage_e9="1000000000",
        salt="1739492960034",
        signed_at_millis=1739494777020
    )

    signature = sign.adjust_leverage(payload)

#     # the expected signature is created using TS SDK
    assert signature == "AFzye47AblhTR0x1/SY8iYzixo+xmZFWxjcuF3+ePnC2Iv/9Gv2liRE8qas3u4WxqiaDXA1BuvoC+kh5wX+Y3AMaLlCCRHvA/vqN6nW+Jej9zfcjU6QQk7w6d0FUK1vlwg=="



@pytest.mark.asyncio
async def test_adjust_margin_signature():

    payload = {
        "ids_id":"0x38a9b1c542212bccc84bdf315afda5a15f00662520c25b31b47cdf87a3705ac9",
        "account_address":"0x76a831f3a961579e5fd34b6cda412409dd41ba18fe0f10e149ecdad413af4050",
        "symbol":"ETH-PERP",
        "add":True,
        "amount_e9":"1000000000000000000",
        "salt":"1739493867512",
        "signed_at_millis":1739495292517
        }


    signature = sign.adjust_margin(payload)

    # the expected signature is created using TS SDK
    assert signature == "AO7Tz9GZq8m8CnIjisUw+rdZbx8wOnPxzlDMyKcPUrruFPi/LbTuuTbump/AJoI4TaX7vpNlCxEiTEkHPu6flggaLlCCRHvA/vqN6nW+Jej9zfcjU6QQk7w6d0FUK1vlwg=="



@pytest.mark.asyncio
async def test_authorize_user_signature():
    payload = AccountAuthorizationRequestSignedFields(
        ids_id="0x38a9b1c542212bccc84bdf315afda5a15f00662520c25b31b47cdf87a3705ac9",
        account_address="0x76a831f3a961579e5fd34b6cda412409dd41ba18fe0f10e149ecdad413af4050",
        authorized_account_address="0x01b51fb67313ae92f57418bc16aebd6134a6f39c388a77092cfdc4c639863d68",
        salt="1739494582583",
        signed_at_millis=1739496172873
    )

    signature = sign.authorize_account(payload, is_authorize=True)

    # the expected signature is created using TS SDK
    assert signature == "ALbxmAd6CE7jhJdiZ5VmSwen1tGOCYe0mr68WrdbHrg4kOIbvBK0JYaHwPAjbwQm9O5kHCbZr36kWUuuVMhEMAYaLlCCRHvA/vqN6nW+Jej9zfcjU6QQk7w6d0FUK1vlwg=="

@pytest.mark.asyncio
async def test_close_position_signature():
    payload = {
        "ids_id":"0x38a9b1c542212bccc84bdf315afda5a15f00662520c25b31b47cdf87a3705ac9",
        "account_address":"0x76a831f3a961579e5fd34b6cda412409dd41ba18fe0f10e149ecdad413af4050",
        "symbol": 'ETH-PERP',
        "is_isolated":True,
        "salt":"1739494221202",
        "signed_at_millis":1739496381885
    }

    signature = sign.close_position(payload)

    # the expected signature is created using TS SDK
    assert signature == "ANJZqVskVwzq2urppJBDWYuPafeEMIVla4hnpAzv3A/WvNEMR6j6SlXfqdoDdtWIT9kdvePaA2dp9IYRWul2yQcaLlCCRHvA/vqN6nW+Jej9zfcjU6QQk7w6d0FUK1vlwg=="
