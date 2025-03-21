# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.asset_config import AssetConfig

class TestAssetConfig(unittest.TestCase):
    """AssetConfig unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> AssetConfig:
        """Test AssetConfig
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `AssetConfig`
        """
        model = AssetConfig()
        if include_optional:
            return AssetConfig(
                asset_type = '',
                symbol = 'USDC',
                decimals = 6,
                weight = '1.0',
                margin_available = True
            )
        else:
            return AssetConfig(
                asset_type = '',
                symbol = 'USDC',
                decimals = 6,
                weight = '1.0',
                margin_available = True,
        )
        """

    def testAssetConfig(self):
        """Test AssetConfig"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
