# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.campaign_metadata import CampaignMetadata

class TestCampaignMetadata(unittest.TestCase):
    """CampaignMetadata unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> CampaignMetadata:
        """Test CampaignMetadata
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `CampaignMetadata`
        """
        model = CampaignMetadata()
        if include_optional:
            return CampaignMetadata(
                status = 'ACTIVE',
                campaign_name = 'Trade and Earn',
                parent_campaign_name = '',
                start_date = 1724121094751,
                end_date = 1724121094751
            )
        else:
            return CampaignMetadata(
                status = 'ACTIVE',
                campaign_name = 'Trade and Earn',
                parent_campaign_name = '',
                start_date = 1724121094751,
                end_date = 1724121094751,
        )
        """

    def testCampaignMetadata(self):
        """Test CampaignMetadata"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
