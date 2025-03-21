# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.market import Market

class TestMarket(unittest.TestCase):
    """Market unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Market:
        """Test Market
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Market`
        """
        model = Market()
        if include_optional:
            return Market(
                symbol = 'BTC-PERP',
                status = 'ACTIVE',
                base_asset_symbol = '',
                base_asset_name = '',
                base_asset_decimals = 56,
                step_size_e9 = '10000000',
                tick_size_e9 = '10000000',
                min_order_quantity_e9 = '20000000',
                max_limit_order_quantity_e9 = '25000000000',
                max_market_order_quantity_e9 = '250000000000',
                min_order_price_e9 = '10000000',
                max_order_price_e9 = '10000000000',
                maintenance_margin_ratio_e9 = '30000000',
                initial_margin_ratio_e9 = '45000000',
                insurance_pool_ratio_e9 = '3000000',
                default_leverage_e9 = '3000000000',
                max_notional_at_open_e9 = [
                    ''
                    ],
                min_trade_quantity_e9 = '10000000',
                max_trade_quantity_e9 = '100000000000',
                min_trade_price_e9 = '50000000000',
                max_trade_price_e9 = '50000000000000',
                max_funding_rate_e9 = '1000000',
                default_maker_fee_e9 = '150000',
                default_taker_fee_e9 = '550000',
                insurance_pool_address = '',
                fee_pool_address = '',
                trading_start_time_at_millis = '1724121094751',
                mtb_long_e9 = '20000000',
                mtb_short_e9 = '20000000',
                delisting_price_e9 = '200000000',
                isolated_only = True
            )
        else:
            return Market(
                symbol = 'BTC-PERP',
                status = 'ACTIVE',
                base_asset_symbol = '',
                base_asset_name = '',
                base_asset_decimals = 56,
                step_size_e9 = '10000000',
                tick_size_e9 = '10000000',
                min_order_quantity_e9 = '20000000',
                max_limit_order_quantity_e9 = '25000000000',
                max_market_order_quantity_e9 = '250000000000',
                min_order_price_e9 = '10000000',
                max_order_price_e9 = '10000000000',
                maintenance_margin_ratio_e9 = '30000000',
                initial_margin_ratio_e9 = '45000000',
                insurance_pool_ratio_e9 = '3000000',
                default_leverage_e9 = '3000000000',
                max_notional_at_open_e9 = [
                    ''
                    ],
                min_trade_quantity_e9 = '10000000',
                max_trade_quantity_e9 = '100000000000',
                min_trade_price_e9 = '50000000000',
                max_trade_price_e9 = '50000000000000',
                max_funding_rate_e9 = '1000000',
                default_maker_fee_e9 = '150000',
                default_taker_fee_e9 = '550000',
                insurance_pool_address = '',
                fee_pool_address = '',
                trading_start_time_at_millis = '1724121094751',
                mtb_long_e9 = '20000000',
                mtb_short_e9 = '20000000',
                delisting_price_e9 = '200000000',
                isolated_only = True,
        )
        """

    def testMarket(self):
        """Test Market"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
