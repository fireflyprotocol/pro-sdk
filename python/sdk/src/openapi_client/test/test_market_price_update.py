# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.market_price_update import MarketPriceUpdate

class TestMarketPriceUpdate(unittest.TestCase):
    """MarketPriceUpdate unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> MarketPriceUpdate:
        """Test MarketPriceUpdate
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `MarketPriceUpdate`
        """
        model = MarketPriceUpdate()
        if include_optional:
            return MarketPriceUpdate(
                symbol = '',
                price_e9 = '',
                source = 'Market',
                updated_at_millis = 56
            )
        else:
            return MarketPriceUpdate(
                symbol = '',
                price_e9 = '',
                source = 'Market',
                updated_at_millis = 56,
        )
        """

    def testMarketPriceUpdate(self):
        """Test MarketPriceUpdate"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
