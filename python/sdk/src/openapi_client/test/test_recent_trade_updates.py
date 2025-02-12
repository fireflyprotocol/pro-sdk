# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.recent_trade_updates import RecentTradeUpdates

class TestRecentTradeUpdates(unittest.TestCase):
    """RecentTradeUpdates unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> RecentTradeUpdates:
        """Test RecentTradeUpdates
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `RecentTradeUpdates`
        """
        model = RecentTradeUpdates()
        if include_optional:
            return RecentTradeUpdates(
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
            return RecentTradeUpdates(
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

    def testRecentTradeUpdates(self):
        """Test RecentTradeUpdates"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
