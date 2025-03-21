# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.active_order_update import ActiveOrderUpdate

class TestActiveOrderUpdate(unittest.TestCase):
    """ActiveOrderUpdate unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> ActiveOrderUpdate:
        """Test ActiveOrderUpdate
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `ActiveOrderUpdate`
        """
        model = ActiveOrderUpdate()
        if include_optional:
            return ActiveOrderUpdate(
                order_hash = '',
                client_order_id = '',
                symbol = '',
                account_address = '',
                price_e9 = '',
                quantity_e9 = '',
                filled_quantity_e9 = '',
                side = 'LONG',
                leverage_e9 = '',
                is_isolated = True,
                salt = '',
                expires_at_millis = 56,
                signed_at_millis = 56,
                type = 'LIMIT',
                reduce_only = True,
                post_only = True,
                time_in_force = 'GTT',
                trigger_price_e9 = '',
                status = 'OPEN',
                self_trade_prevention_type = 'TAKER',
                created_at_millis = 56,
                updated_at_millis = 56
            )
        else:
            return ActiveOrderUpdate(
                order_hash = '',
                symbol = '',
                account_address = '',
                price_e9 = '',
                quantity_e9 = '',
                filled_quantity_e9 = '',
                side = 'LONG',
                leverage_e9 = '',
                is_isolated = True,
                salt = '',
                expires_at_millis = 56,
                signed_at_millis = 56,
                type = 'LIMIT',
                reduce_only = True,
                post_only = True,
                time_in_force = 'GTT',
                status = 'OPEN',
                self_trade_prevention_type = 'TAKER',
                created_at_millis = 56,
                updated_at_millis = 56,
        )
        """

    def testActiveOrderUpdate(self):
        """Test ActiveOrderUpdate"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
