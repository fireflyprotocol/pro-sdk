# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.recent_trades_updates import RecentTradesUpdates

class TestRecentTradesUpdates(unittest.TestCase):
    """RecentTradesUpdates unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> RecentTradesUpdates:
        """Test RecentTradesUpdates
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `RecentTradesUpdates`
        """
        model = RecentTradesUpdates()
        if include_optional:
            return RecentTradesUpdates(
                recent_trades = [
                    openapi_client.models.recent_trades_update.RecentTradesUpdate(
                        id = 56, 
                        market_address = '', 
                        price_e9 = '', 
                        quantity_e9 = '', 
                        quote_quantity_e9 = '', 
                        side = 'LONG', 
                        updated_at_utc_millis = 56, )
                    ]
            )
        else:
            return RecentTradesUpdates(
                recent_trades = [
                    openapi_client.models.recent_trades_update.RecentTradesUpdate(
                        id = 56, 
                        market_address = '', 
                        price_e9 = '', 
                        quantity_e9 = '', 
                        quote_quantity_e9 = '', 
                        side = 'LONG', 
                        updated_at_utc_millis = 56, )
                    ],
        )
        """

    def testRecentTradesUpdates(self):
        """Test RecentTradesUpdates"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
