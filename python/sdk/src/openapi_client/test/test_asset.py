# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.asset import Asset

class TestAsset(unittest.TestCase):
    """Asset unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Asset:
        """Test Asset
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Asset`
        """
        model = Asset()
        if include_optional:
            return Asset(
                symbol = '',
                quantity_e9 = '6000000000',
                effective_balance_e9 = '6000000000',
                max_withdraw_quantity_e9 = '3000000000',
                last_updated_at_millis = 1627872345678
            )
        else:
            return Asset(
                symbol = '',
                quantity_e9 = '6000000000',
                effective_balance_e9 = '6000000000',
                max_withdraw_quantity_e9 = '3000000000',
                last_updated_at_millis = 1627872345678,
        )
        """

    def testAsset(self):
        """Test Asset"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
