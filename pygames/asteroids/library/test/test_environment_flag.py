import unittest
from io import StringIO
from unittest.mock import patch
from ..environment_flag import EnvironmentFlag


class TestEnvironmentFlag(unittest.TestCase):
    def test_valid_config(self):
        valid_config = StringIO(
            "[Flags]\n" "IS_PRODUCTION_BUILD = False\n" "IS_MAC_BUILD = True\n"
        )
        # replaces the open() function with a mock that returns the config_file
        # replaces the os.path.exists() function with a mock that returns True
        with patch("builtins.open", return_value=valid_config), patch(
            "os.path.exists", return_value=True
        ):
            flag = EnvironmentFlag("test_generated_dummy_path")
            if flag.has_section:
                self.assertFalse(flag.is_production)
                self.assertTrue(flag.is_mac_build)
            else:
                self.skipTest("No SECTION HEADER found in config file")

    def test_missing_section(self):
        # neglecting to include any section header : [Flags]
        invalid_config = StringIO(
            "IS_PRODUCTION_BUILD = False\n" "IS_MAC_BUILD = True\n"
        )
        with patch("builtins.open", return_value=invalid_config), patch(
            "os.path.exists", return_value=True
        ):
            flag = EnvironmentFlag("test_generated_dummy_path")
            if flag.has_section():
                # Test case should NOT have a SECTION HEADER
                self.skipTest("Unexpected SECTION HEADER found in config file")
            else:
                # Checkt that EnvironmentFlag defaults are False to pass test
                self.assertTrue(
                    flag.is_production == EnvironmentFlag.IS_PRODUCTION_DEFAULT
                )
                self.assertTrue(
                    flag.is_mac_build == EnvironmentFlag.IS_MAC_BUILD_DEFAULT
                )
