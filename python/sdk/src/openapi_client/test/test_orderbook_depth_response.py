# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.orderbook_depth_response import OrderbookDepthResponse

class TestOrderbookDepthResponse(unittest.TestCase):
    """OrderbookDepthResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> OrderbookDepthResponse:
        """Test OrderbookDepthResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `OrderbookDepthResponse`
        """
        model = OrderbookDepthResponse()
        if include_optional:
            return OrderbookDepthResponse(
                symbol = '1193046',
                last_update_id = 56,
                last_updated_at_utc_millis = 56,
                response_sent_at_utc_millis = 56,
                best_bid_price_e9 = '',
                best_bid_quantity_e9 = '',
                best_ask_price_e9 = '',
                best_ask_quantity_e9 = '',
                bids_e9 = [["4000000000","431000000000"]],
                asks_e9 = [["4000002000","12000000000"]]
            )
        else:
            return OrderbookDepthResponse(
                symbol = '1193046',
                last_update_id = 56,
                last_updated_at_utc_millis = 56,
                response_sent_at_utc_millis = 56,
                best_bid_price_e9 = '',
                best_bid_quantity_e9 = '',
                best_ask_price_e9 = '',
                best_ask_quantity_e9 = '',
                bids_e9 = [["4000000000","431000000000"]],
                asks_e9 = [["4000002000","12000000000"]],
        )
        """

    def testOrderbookDepthResponse(self):
        """Test OrderbookDepthResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
