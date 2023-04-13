import configparser
import logging


class EnvironmentFlag:
    """
    Class for managing environment flags from a configuration file
    """

    CONFIG_FILE_PATH: str = "config.ini"
    SECTION_FLAGS: str = "Flags"
    # default values if config file is missing or malformed
    IS_PRODUCTION_DEFAULT: bool = False
    IS_MAC_BUILD_DEFAULT: bool = False

    def __init__(self, config_file_path: str = CONFIG_FILE_PATH):
        self.config = configparser.ConfigParser()
        # set default values to False
        self.is_production = EnvironmentFlag.IS_PRODUCTION_DEFAULT
        self.is_mac_build = EnvironmentFlag.IS_MAC_BUILD_DEFAULT

        try:
            config_content = self.config.read(config_file_path)
            if config_content == []:
                logging.warning("No config file found at {config_file_path}")
                return
            if self.has_section():
                try:
                    self.is_production = self.config.getboolean(
                        EnvironmentFlag.SECTION_FLAGS, "IS_PRODUCTION_BUILD"
                    )
                    self.is_mac_build = self.config.getboolean(
                        EnvironmentFlag.SECTION_FLAGS, "IS_MAC_BUILD"
                    )
                except configparser.NoOptionError as e:
                    logging.warning(f"Missing option in config file : {e}")
            else:
                logging.warning("No SECTION HEADER found in config file")
        except configparser.MissingSectionHeaderError as e:
            logging.warning(f"Missing SECTION HEADER in config file : {e}")
            # set default values in case of an error
        print(f"IS_PRODUCTION_BUILD: {self.is_production}")
        print(f"IS_MAC_BUILD: {self.is_mac_build}")

    def has_section(self) -> bool:
        return self.config.has_section(EnvironmentFlag.SECTION_FLAGS)
