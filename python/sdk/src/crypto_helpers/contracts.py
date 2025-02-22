from typing import Dict, Any

class ProContracts:
    def __init__(self, contract_config: Dict[str, Any]) -> None:
        """
        Initialize the Pro contract instance.

        Args:
            contract_config (Dict): Dictionary containing contract addresses.

        Returns:
            A tuple containing the signed login request and the signature.
        """
        self.contract_config = contract_config


    def get_external_data_store_id(self) -> str:
        """
        Returns object ID of external data store.
        """
        return self.contract_config["ExternalDataStore"]
  
    def get_internal_data_store_id(self) -> str:
        """
        Returns object ID of internal data store.
        """
        return self.contract_config["InternalDataStore"]

    def get_package_id(self) -> str:
        """
        Returns the package id.
        """
        return self.contract_config["Package"]

    def get_supported_token(self, symbol: str):
        """
        Returns the details of provided supported token
        """

        if symbol in self.contract_config["SupportedAssets"]:
            return self.contract_config["SupportedAssets"][symbol]
        else:
            raise Exception("{} is not in supported assets list".format(symbol))
        

    def get_supported_token_type(self, symbol: str):
        """
        Returns the token type provided supported token
        """
        token = self.get_supported_token(symbol)
        
        return token["coinType"]
