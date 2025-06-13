import base64
from hashlib import blake2b

from crypto_helpers.wallet import SuiWallet
from crypto_helpers.contracts import ProContracts
from crypto_helpers.utils import get_coin_having_balance, rpc_unsafe_moveCall, rpc_sui_executeTransactionBlock

class ProRpcCalls:
    def __init__(self, sui_wallet: SuiWallet, contracts: ProContracts, url: str):
        self.sui_wallet = sui_wallet
        self.contracts = contracts
        self.rpc_url = url

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

        result = self.sui_wallet.private_key.sign(hash)
        temp = bytearray()
        temp.append(0)
        temp.extend(result)
        temp.extend(self.sui_wallet.public_key_bytes)
        res = base64.b64encode(temp)
        return res.decode()
    
    def deposit_to_asset_bank(self, coin_symbol: str, amount: int, destination: str = None):
        """
        Deposits the provided coin of provided amount
        into the external asset bank
    
        Args:

            coin_symbol (str): The symbol of the coin being deposited
            amount (uint): The amount to be deposited in the units of the coin being deposited (for example, 1 USDC == 1000000)
            destination (str): Optional destination account to which funds are being deposited.
                               By default, funds are always deposited to the depositor's account

        """

        coin_type = self.contracts.get_supported_token_type(coin_symbol)

        # get the coin for deposit
        coin_id = get_coin_having_balance(
            user_address=self.sui_wallet.sui_address,
            coin_type=coin_type,
            balance=amount,
            url=self.rpc_url,
            exact_match=False
        )


        # prepare the transaction arguments
        move_function_params = [
                    self.contracts.get_external_data_store_id(),
                    coin_symbol,
                    self.sui_wallet.sui_address if destination == None else destination,
                    str(amount),
                    coin_id
                ]
                
        tx_bytes = rpc_unsafe_moveCall(
            url=self.rpc_url,
            params=move_function_params,
            function_name='deposit_to_asset_bank',
            function_library='exchange',
            userAddress=self.sui_wallet.sui_address,
            packageId=self.contracts.get_package_id(),
            gasBudget=10000000,
            typeArguments=[ coin_type ]
        )

        signature = self.sign_tx(tx_bytes)
        res = rpc_sui_executeTransactionBlock(self.rpc_url, tx_bytes, signature)
        try:
            success = res["result"]["effects"]["status"]["status"] == "success"
            return success, res
        except Exception as e:
            return False , res
        