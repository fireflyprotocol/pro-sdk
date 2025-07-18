# coding: utf-8

"""
    Bluefin API

    Bluefin API

    The version of the OpenAPI document: 1.0.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from openapi_client.models.referee_onboard_response import RefereeOnboardResponse

class TestRefereeOnboardResponse(unittest.TestCase):
    """RefereeOnboardResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> RefereeOnboardResponse:
        """Test RefereeOnboardResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `RefereeOnboardResponse`
        """
        model = RefereeOnboardResponse()
        if include_optional:
            return RefereeOnboardResponse(
                message = 'onboarded successfully'
            )
        else:
            return RefereeOnboardResponse(
                message = 'onboarded successfully',
        )
        """

    def testRefereeOnboardResponse(self):
        """Test RefereeOnboardResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
