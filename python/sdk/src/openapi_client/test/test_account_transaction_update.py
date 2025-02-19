# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.account_transaction_update import AccountTransactionUpdate

class TestAccountTransactionUpdate(unittest.TestCase):
    """AccountTransactionUpdate unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> AccountTransactionUpdate:
        """Test AccountTransactionUpdate
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `AccountTransactionUpdate`
        """
        model = AccountTransactionUpdate()
        if include_optional:
            return AccountTransactionUpdate(
                symbol = '',
                transaction_type = 'DEPOSIT',
                amount_e9 = '',
                asset_symbol = '',
                trade_id = ''
            )
        else:
            return AccountTransactionUpdate(
                transaction_type = 'DEPOSIT',
                amount_e9 = '',
        )
        """

    def testAccountTransactionUpdate(self):
        """Test AccountTransactionUpdate"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
