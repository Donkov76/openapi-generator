# coding: utf-8

"""
    OpenAPI Petstore

    This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\  # noqa: E501

    The version of the OpenAPI document: 1.0.0
    Generated by: https://openapi-generator.tech
"""


from __future__ import absolute_import

import unittest

import petstore_api
from petstore_api.api.fake_api import FakeApi  # noqa: E501


class TestFakeApi(unittest.TestCase):
    """FakeApi unit test stubs"""

    def setUp(self):
        self.api = FakeApi()  # noqa: E501

    def tearDown(self):
        pass

    def test_create_xml_item(self):
        """Test case for create_xml_item

        creates an XmlItem  # noqa: E501
        """
        pass

    def test_boolean(self):
        """Test case for boolean

        """
        endpoint = self.api.boolean
        assert endpoint.openapi_types['body'] == (bool,)
        assert endpoint.settings['response_type'] == (bool,)

    def test_string(self):
        """Test case for string

        """
        from petstore_api.model_utils import str
        endpoint = self.api.string
        assert endpoint.openapi_types['body'] == (str,)
        assert endpoint.settings['response_type'] == (str,)

    def test_object_model_with_ref_props(self):
        """Test case for object_model_with_ref_props

        """
        from petstore_api.model import object_model_with_ref_props
        endpoint = self.api.object_model_with_ref_props
        assert endpoint.openapi_types['body'] == (object_model_with_ref_props.ObjectModelWithRefProps,)
        assert endpoint.settings['response_type'] == (object_model_with_ref_props.ObjectModelWithRefProps,)

    def test_string_enum(self):
        """Test case for string_enum

        """
        from petstore_api.model import string_enum
        endpoint = self.api.string_enum
        assert endpoint.openapi_types['body'] == (string_enum.StringEnum,)
        assert endpoint.settings['response_type'] == (string_enum.StringEnum,)

    def test_array_model(self):
        """Test case for array_model

        """
        from petstore_api.model import animal_farm
        endpoint = self.api.array_model
        assert endpoint.openapi_types['body'] == (animal_farm.AnimalFarm,)
        assert endpoint.settings['response_type'] == (animal_farm.AnimalFarm,)

    def test_number_with_validations(self):
        """Test case for number_with_validations

        """
        from petstore_api.model import number_with_validations
        endpoint = self.api.number_with_validations
        assert endpoint.openapi_types['body'] == (number_with_validations.NumberWithValidations,)
        assert endpoint.settings['response_type'] == (number_with_validations.NumberWithValidations,)

    def test_test_body_with_file_schema(self):
        """Test case for test_body_with_file_schema

        """
        pass

    def test_test_body_with_query_params(self):
        """Test case for test_body_with_query_params

        """
        pass

    def test_test_client_model(self):
        """Test case for test_client_model

        To test \"client\" model  # noqa: E501
        """
        pass

    def test_test_endpoint_enums_length_one(self):
        """Test case for test_endpoint_enums_length_one

        """
        # when we omit the required enums of length one, they are still set
        endpoint = self.api.test_endpoint_enums_length_one
        import six
        if six.PY3:
            from unittest.mock import patch
        else:
            from mock import patch
        with patch.object(endpoint, 'call_with_http_info') as call_with_http_info:
            endpoint()
            call_with_http_info.assert_called_with(
                _check_input_type=True,
                _check_return_type=True,
                _host_index=None,
                _preload_content=True,
                _request_timeout=None,
                _return_http_data_only=True,
                async_req=False,
                header_number=1.234,
                path_integer=34,
                path_string='hello',
                query_integer=3,
                query_string='brillig'
            )

    def test_test_endpoint_parameters(self):
        """Test case for test_endpoint_parameters

        Fake endpoint for testing various parameters 假端點 偽のエンドポイント 가짜 엔드 포인트   # noqa: E501
        """
        # check that we can access the endpoint's validations
        endpoint = self.api.test_endpoint_parameters
        assert endpoint.validations[('number',)] == {
            'inclusive_maximum': 543.2,
            'inclusive_minimum': 32.1,
        }
        # make sure that an exception is thrown on an invalid value
        keyword_args = dict(
            number=544,  # invalid
            double=100,
            pattern_without_delimiter="abc",
            byte='sample string'
        )
        with self.assertRaises(petstore_api.ApiValueError):
            self.api.test_endpoint_parameters(**keyword_args)

    def test_test_enum_parameters(self):
        """Test case for test_enum_parameters

        To test enum parameters  # noqa: E501
        """
        # check that we can access the endpoint's allowed_values
        endpoint = self.api.test_enum_parameters
        assert endpoint.allowed_values[('enum_query_string',)] == {
            "_ABC": "_abc",
            "-EFG": "-efg",
            "(XYZ)": "(xyz)"
        }
        # make sure that an exception is thrown on an invalid value
        keyword_args = dict(enum_query_string="bad value")
        with self.assertRaises(petstore_api.ApiValueError):
            self.api.test_enum_parameters(**keyword_args)

    def test_test_group_parameters(self):
        """Test case for test_group_parameters

        Fake endpoint to test group parameters (optional)  # noqa: E501
        """
        pass

    def test_test_inline_additional_properties(self):
        """Test case for test_inline_additional_properties

        test inline additionalProperties  # noqa: E501
        """
        pass

    def test_test_json_form_data(self):
        """Test case for test_json_form_data

        test json serialization of form data  # noqa: E501
        """
        pass


if __name__ == '__main__':
    unittest.main()
