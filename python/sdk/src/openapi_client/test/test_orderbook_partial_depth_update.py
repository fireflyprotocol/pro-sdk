# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.orderbook_partial_depth_update import OrderbookPartialDepthUpdate

class TestOrderbookPartialDepthUpdate(unittest.TestCase):
    """OrderbookPartialDepthUpdate unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> OrderbookPartialDepthUpdate:
        """Test OrderbookPartialDepthUpdate
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `OrderbookPartialDepthUpdate`
        """
        model = OrderbookPartialDepthUpdate()
        if include_optional:
            return OrderbookPartialDepthUpdate(
                updated_at_millis = 56,
                symbol = '',
                bids_e9 = [
                    [["10000000000","10000000000"],["9000000000","9000000000"]]
                    ],
                asks_e9 = [
                    [["10000000000","10000000000"],["9000000000","9000000000"]]
                    ],
                orderbook_update_id = 56,
                depth_level = '5'
            )
        else:
            return OrderbookPartialDepthUpdate(
                updated_at_millis = 56,
                symbol = '',
                bids_e9 = [
                    [["10000000000","10000000000"],["9000000000","9000000000"]]
                    ],
                asks_e9 = [
                    [["10000000000","10000000000"],["9000000000","9000000000"]]
                    ],
                orderbook_update_id = 56,
                depth_level = '5',
        )
        """

    def testOrderbookPartialDepthUpdate(self):
        """Test OrderbookPartialDepthUpdate"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
