# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.open_order_response import OpenOrderResponse

class TestOpenOrderResponse(unittest.TestCase):
    """OpenOrderResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> OpenOrderResponse:
        """Test OpenOrderResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `OpenOrderResponse`
        """
        model = OpenOrderResponse()
        if include_optional:
            return OpenOrderResponse(
                order_hash = '',
                client_order_id = 'company1_order1',
                symbol = '',
                account_address = '',
                price_e9 = '452400000',
                quantity_e9 = '452400000',
                side = 'LONG',
                leverage_e9 = '4000000000',
                is_isolated = True,
                salt = '123335432',
                expires_at_millis = 123456734567,
                signed_at_millis = 1234567856,
                type = 'LIMIT',
                reduce_only = True,
                post_only = True,
                time_in_force = 'GTT',
                trigger_price_e9 = '452400000',
                filled_quantity_e9 = '300000000',
                status = 'OPEN',
                self_trade_prevention_type = 'TAKER',
                order_time_at_millis = 56,
                last_updated_at_millis = 56
            )
        else:
            return OpenOrderResponse(
                order_hash = '',
                symbol = '',
                account_address = '',
                price_e9 = '452400000',
                quantity_e9 = '452400000',
                side = 'LONG',
                leverage_e9 = '4000000000',
                is_isolated = True,
                salt = '123335432',
                expires_at_millis = 123456734567,
                signed_at_millis = 1234567856,
                type = 'LIMIT',
                reduce_only = True,
                post_only = True,
                time_in_force = 'GTT',
                filled_quantity_e9 = '300000000',
                status = 'OPEN',
                self_trade_prevention_type = 'TAKER',
                order_time_at_millis = 56,
                last_updated_at_millis = 56,
        )
        """

    def testOpenOrderResponse(self):
        """Test OpenOrderResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
