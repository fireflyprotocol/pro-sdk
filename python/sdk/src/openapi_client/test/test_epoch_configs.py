# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.epoch_configs import EpochConfigs

class TestEpochConfigs(unittest.TestCase):
    """EpochConfigs unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> EpochConfigs:
        """Test EpochConfigs
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `EpochConfigs`
        """
        model = EpochConfigs()
        if include_optional:
            return EpochConfigs(
                campaign_name = 'Trade and Earn',
                epoch_duration = 604800000,
                sui_rewards_allocation = '200000000000',
                blue_rewards_allocation = '5000000000',
                cash_rewards_allocation = '200000000000',
                config = { }
            )
        else:
            return EpochConfigs(
                campaign_name = 'Trade and Earn',
                epoch_duration = 604800000,
                sui_rewards_allocation = '200000000000',
                blue_rewards_allocation = '5000000000',
                cash_rewards_allocation = '200000000000',
                config = { },
        )
        """

    def testEpochConfigs(self):
        """Test EpochConfigs"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
