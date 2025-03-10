# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.operators import Operators

class TestOperators(unittest.TestCase):
    """Operators unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Operators:
        """Test Operators
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Operators`
        """
        model = Operators()
        if include_optional:
            return Operators(
                admin = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                operator = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                sequencer = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                funding = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                fee = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0'
            )
        else:
            return Operators(
                admin = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                operator = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                sequencer = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                funding = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
                fee = '0xa6b939f57595fed2ae10ae4f3a37a410c94e5bdb7dde2e547022d1fe1b9791d0',
        )
        """

    def testOperators(self):
        """Test Operators"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
