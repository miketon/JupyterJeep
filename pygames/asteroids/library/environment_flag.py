import configparser
import logging
import os
import os.path

class EnvironmentFlag:
    """
    Class for managing environment flags from a configuration file
    """

    # absolute path to this EnvironmentFlag.py file
    CONFIG_FILE_PATH:str = os.path.dirname(os.path.abspath(__file__))
    CONFIG_FILE: str = "config.ini"
    SECTION_FLAGS: str = "Flags"
    IS_PRODUCTION_FLAG: str = "IS_PRODUCTION_BUILD"
    IS_MAC_BUILD_FLAG: str = "IS_MAC_BUILD"
    # default values if config file is missing or malformed
    IS_PRODUCTION_DEFAULT: bool = False
    IS_MAC_BUILD_DEFAULT: bool = False

    def __init__(self, config_file_path: str =""):
        if(config_file_path == ""):
            config_file_path = os.path.join(EnvironmentFlag.CONFIG_FILE_PATH, EnvironmentFlag.CONFIG_FILE)
        self.config = configparser.ConfigParser()
        # set default values to False
        self.is_production = EnvironmentFlag.IS_PRODUCTION_DEFAULT
        self.is_mac_build = EnvironmentFlag.IS_MAC_BUILD_DEFAULT
        self._read_config(config_file_path)

    def _read_config(self, config_file_path: str):
        try:
            config_content = self.config.read(config_file_path)
            if config_content == []:
                logging.warning(f"No config file found at {config_file_path}")
                return
            if self.has_section():
                try:
                    self.is_production = self.config.getboolean(
                        EnvironmentFlag.SECTION_FLAGS,
                        EnvironmentFlag.IS_PRODUCTION_FLAG,
                    )
                    self.is_mac_build = self.config.getboolean(
                        EnvironmentFlag.SECTION_FLAGS, EnvironmentFlag.IS_MAC_BUILD_FLAG
                    )
                except configparser.NoOptionError as e:
                    logging.warning(f"Missing option in config file : {e}")
                except ValueError as e:
                    raise InvalidFlagValueError("Invalid value in config file : {e}")
            else:
                logging.warning(
                    f"No SECTION HEADER found in config file at {config_file_path}"
                )
        except configparser.MissingSectionHeaderError as e:
            logging.warning(f"Missing SECTION HEADER in config file : {e}")
            # set default values in case of an error
        print(f"IS_PRODUCTION_BUILD: {self.is_production}")
        print(f"IS_MAC_BUILD: {self.is_mac_build}")

    def has_section(self) -> bool:
        return self.config.has_section(EnvironmentFlag.SECTION_FLAGS)


class InvalidFlagValueError(ValueError):
    """
    Exception raised when an invalid option value is passed to a flag
    """

    pass
