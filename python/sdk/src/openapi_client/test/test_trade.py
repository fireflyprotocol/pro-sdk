# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.trade import Trade

class TestTrade(unittest.TestCase):
    """Trade unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Trade:
        """Test Trade
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Trade`
        """
        model = Trade()
        if include_optional:
            return Trade(
                trade_id = '',
                client_order_id = '25851813',
                symbol = '',
                order_hash = '',
                trade_type = 'ORDER',
                side = 'LONG',
                is_maker = False,
                price_e9 = '7819010000000',
                quantity_e9 = '20000000000',
                quote_quantity_e9 = '150000000000000',
                realized_pnl_e9 = '-9000000000',
                position_side = 'LONG',
                trading_fee_e9 = '-780000000',
                trading_fee_asset = 'USDC',
                gas_fee_e9 = 0,
                gas_fee_asset = 'USDC',
                executed_at_utc_milli = 1569514978020
            )
        else:
            return Trade(
                trade_id = '',
                trade_type = 'ORDER',
                side = 'LONG',
                is_maker = False,
                price_e9 = '7819010000000',
                quantity_e9 = '20000000000',
                quote_quantity_e9 = '150000000000000',
                position_side = 'LONG',
                trading_fee_e9 = '-780000000',
                trading_fee_asset = 'USDC',
                executed_at_utc_milli = 1569514978020,
        )
        """

    def testTrade(self):
        """Test Trade"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
