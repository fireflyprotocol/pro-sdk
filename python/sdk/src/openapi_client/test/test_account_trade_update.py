# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.account_trade_update import AccountTradeUpdate

class TestAccountTradeUpdate(unittest.TestCase):
    """AccountTradeUpdate unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> AccountTradeUpdate:
        """Test AccountTradeUpdate
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `AccountTradeUpdate`
        """
        model = AccountTradeUpdate()
        if include_optional:
            return AccountTradeUpdate(
                trade_id = '',
                client_order_id = '',
                symbol = '',
                order_hash = '',
                type = 'ORDER',
                order_side = 'LONG',
                is_maker = True,
                price_e9 = '',
                quantity_e9 = '',
                quote_quantity_e9 = '',
                realized_pnl_e9 = '',
                position_side = 'LONG',
                trading_fee_e9 = '',
                trading_fee_asset_symbol = '',
                executed_at_millis = 56
            )
        else:
            return AccountTradeUpdate(
                trade_id = '',
                symbol = '',
                order_hash = '',
                type = 'ORDER',
                order_side = 'LONG',
                is_maker = True,
                price_e9 = '',
                quantity_e9 = '',
                quote_quantity_e9 = '',
                realized_pnl_e9 = '',
                position_side = 'LONG',
                trading_fee_e9 = '',
                trading_fee_asset_symbol = '',
                executed_at_millis = 56,
        )
        """

    def testAccountTradeUpdate(self):
        """Test AccountTradeUpdate"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
