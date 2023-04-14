import unittest
from unittest.mock import patch
import sys
from io import StringIO
from contextlib import contextmanager
from ..environment_flag import EnvironmentFlag, InvalidFlagValueError
import random


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
                # check that mock config has been toggled from DEFAULT (False)
                self.assertTrue(
                    flag.is_production == EnvironmentFlag.IS_PRODUCTION_DEFAULT
                )
                self.assertTrue(
                    flag.is_mac_build != EnvironmentFlag.IS_MAC_BUILD_DEFAULT
                )
            else:
                self.skipTest("No SECTION HEADER found in config file")

    def test_no_config_file(self):
        # replaces config.read() function with a mock that returns an empty list
        with patch("configparser.ConfigParser.read", return_value=[]):
            flag = EnvironmentFlag("test_generated_dummy_path")
            # to pass test EnvironmentFlag defaults are set to DEFAULT values
            self.assertTrue(flag.is_production == EnvironmentFlag.IS_PRODUCTION_DEFAULT)
            self.assertTrue(flag.is_mac_build == EnvironmentFlag.IS_MAC_BUILD_DEFAULT)

    def test_has_section(self):
        has_section_config = StringIO(
            "[Flags]\n" "IS_PRODUCTION_BUILD = True\n" "IS_MAC_BUILD = True\n"
        )
        with patch("builtins.open", return_value=has_section_config):
            flag = EnvironmentFlag("test_generated_dummy_path")
            self.assertTrue(flag.has_section())

        missing_section_config = StringIO(
            "IS_PRODUCTION_BUILD = True\n" "IS_MAC_BUILD = True\n"
        )
        with patch("builtins.open", return_value=missing_section_config):
            flag = EnvironmentFlag("test_generated_dummy_path")
            self.assertFalse(flag.has_section())

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

    def test_invalid_option_values(self):
        # IS_PRODUCTION_BUILD = 1 is valid, but IS_MAC_BUILD = not_a_bool is not
        invalid_option_config = StringIO(
            "[Flags]\n" "IS_PRODUCTION_BUILD = 1\n" "IS_MAC_BUILD = not_a_bool\n"
        )
        with patch("builtins.open", return_value=invalid_option_config), patch(
            "os.path.exists", return_value=True
        ):
            # Check that EnvironmentFlag raises an error when invalid option
            # values are found in the config file
            # Asserts that the given exception InvalidFlagValueError is raised
            # ... if the expected exception is not raised, the test FAILS
            with self.assertRaises(InvalidFlagValueError):
                EnvironmentFlag("test_generated_dummy_path")

    @staticmethod
    @contextmanager
    def capture_stdout():
        original_stdout = sys.stdout
        sys.stdout = StringIO()
        try:
            yield sys.stdout
        finally:
            sys.stdout = original_stdout

    # @note 🧠 : has "_" prefix so that it is not added to the test suite
    # ... also indicates that this is a helper function
    def _get_stdout(self, func, *args):
        with TestEnvironmentFlag.capture_stdout() as new_stdout:
            func(*args)
            output = new_stdout.getvalue().strip()
        return output

    def test_debug_print_in_non_production_environment(self):
        config_not_production = StringIO(
            "[Flags]\n" "IS_PRODUCTION_BUILD = 0\n" "IS_MAC_BUILD = True\n"
        )

        # Check EnvironmentFlag with is_production = False [debug mode]
        with patch("builtins.open", return_value=config_not_production), patch(
            "os.path.exists", return_value=True
        ):
            flag = EnvironmentFlag("test_generated_dummy_path")
            debug_message = "Kevin has a tiny person inside operating him like a crane"
            random_key = random.randint(-999, 999)
            # get output from the debug_print() function
            output = self._get_stdout(flag.debug_print, debug_message, random_key)
            # check that debug message equals the std output
            self.assertTrue(f"{debug_message} {random_key}" == output)

    def test_debug_print_in_production_environment(self):
        config_is_production = StringIO(
            "[Flags]\n" "IS_PRODUCTION_BUILD = 1\n" "IS_MAC_BUILD = True\n"
        )

        # Check EnvironmentFlag with is_production = True [production mode]
        with patch("builtins.open", return_value=config_is_production), patch(
            "os.path.exists", return_value=True
        ):
            flag = EnvironmentFlag("test_generated_dummy_path")
            debug_message = "Kevin has a tiny person inside operating him like a crane"
            random_key = random.randint(-999, 999)
            # get output from debug_print() function
            output = self._get_stdout(flag.debug_print, debug_message, random_key)
            # verify that output is empty when in production environment
            self.assertTrue(output == "")


""" 
GPT-4 Notes and Comments
************************
"""

""" 
Explain "builtins.open"
************************

`builtins.open` refers to the `open()` function in Python, which is used to open 
a file and returns a file object. The `open()` function is part of the built-in 
functions provided by the Python language, and it is available in the `builtins` 
module. You don't generally need to import or use the `builtins` module directly, 
as its functions and classes are available in the global namespace by default.

The `open()` function has the following syntax:

```python
open(file, mode='r', buffering=-1, encoding=None, errors=None, newline=None, 
closefd=True, opener=None)
```

- `file`: The file path (string, bytes, or a path-like object) that you want to 
open.
- `mode`: An optional string specifying the mode in which the file is opened. 
The default is `'r'` for reading (text mode). Other modes include `'w'` for 
writing (overwriting the file if it exists), `'a'` for appending (writing at the 
end of the file), `'x'` for exclusive creation (fails if the file exists), and 
`'b'` for binary mode (used with other modes, e.g., `'rb'` for reading in binary 
mode).
- Other optional arguments: `buffering`, `encoding`, `errors`, `newline`, 
`closefd`, and `opener` have default values and are used to fine-tune the 
behavior of the file object returned.

Here's an example of using the `open()` function to read the contents of a text 
file:

```python
file_path = "example.txt"

with open(file_path, "r") as file:
    content = file.read()

print(content)
```

In this example, the `open()` function is used in a `with` statement, which 
ensures that the file is properly closed after the block is executed. The file i
s opened in read mode (`"r"`), and its content is read using the `read()` method 
of the file object.

"""

""" 
Tips for Testing with Mocks
**************************

When writing test cases, especially for unit tests, it's important to isolate 
the function or method you're testing from external dependencies. Mocking is a 
technique that allows you to replace these external dependencies with objects 
that simulate their behavior. This helps you to test your code in a controlled 
environment and makes your tests more reliable and deterministic.

To determine which mocks to use, follow these steps:

1. **Identify dependencies**: Start by identifying the external dependencies in 
the function or method you're testing. These dependencies could be other 
functions, methods, or classes that the code interacts with, such as file I/O, 
database connections, or API calls.

2. **Evaluate the need for mocking**: Determine if mocking the dependencies is 
necessary. If the dependency is a simple utility function with no side effects 
or if it's fast and deterministic, you might not need to mock it. However, if 
the dependency can cause side effects, has non-deterministic behavior, or can 
slow down your tests, it's a good candidate for mocking.

In the example you provided, there are two dependencies being mocked:

- `builtins.open`: This function is used for reading and writing files. In a 
test environment, it's better to avoid actual file I/O to ensure tests run 
quickly and don't rely on the filesystem's state.
- `os.path.exists`: This function checks if a file or directory exists. Again, 
it's better to avoid interacting with the filesystem in tests to ensure test 
isolation and to avoid any side effects.

3. **Choose the right mocks**: Once you've identified the dependencies to mock, 
choose the appropriate mock objects to replace them. Python's `unittest.mock` 
library provides several useful classes and functions for this purpose, such as 
`Mock`, `MagicMock`, and `patch`.

In the example, the `patch` function is used to temporarily replace the 
`builtins.open` and `os.path.exists` functions within the `with` block. The 
`return_value` argument specifies the value that the mock should return when 
called.

```python
with patch("builtins.open", return_value=valid_config), patch(
    "os.path.exists", return_value=True
):
```

4. **Configure the mock behavior**: Set up the desired behavior for your mock 
objects. This can include specifying return values, raising exceptions, or even 
simulating side effects. In the example, the `return_value` argument is used to 
configure the behavior of the mock objects.

5. **Write and run your tests**: With your mocks in place, write the test cases 
to exercise the code under test. Make sure your tests cover different scenarios, 
edge cases, and possible input values. Finally, run the tests to ensure your 
code behaves as expected.

Remember that mocking should be used judiciously. Overuse of mocks can lead to 
tests that are difficult to understand and maintain. It's essential to strike a 
balance between test isolation and test complexity.

"""

""" 
Explain the difference between patch, Mock, and MagicMock
********************************************************

`patch`, `Mock`, and `MagicMock` are components of the `unittest.mock` library 
in Python, which is used for creating mock objects in unit tests. Each of these 
components serves a different purpose:

1. **patch**: `patch` is a function (also available as a context manager and a 
decorator) that temporarily replaces a specified object with a mock object 
during the execution of a test. It is typically used to replace external 
dependencies, such as functions or classes, with mock objects to isolate the 
code under test. When the `patch` block or decorated function exits, the 
original object is restored.

Example of using `patch` as a context manager:

```python
from unittest.mock import patch

def test_my_function():
    with patch("module_under_test.SomeClass") as mock_class:
        # The original SomeClass is replaced within this block
        # Perform your test here
        pass
    # The original SomeClass is restored after the block
```

2. **Mock**: `Mock` is a class that allows you to create flexible mock objects 
that can be used to replace real objects in your tests. With a `Mock` object, 
you can configure its behavior, such as specifying return values or side effects 
when its methods are called. You can also make assertions about how the mock was 
called, such as checking the number of times a method was called or the arguments it was called with.

Example of using `Mock`:

```python
from unittest.mock import Mock

def test_my_function():
    mock_object = Mock()
    mock_object.some_method.return_value = "some value"
    
    result = my_function(mock_object)
    
    mock_object.some_method.assert_called_once()
    assert result == "expected result"
```

3. **MagicMock**: `MagicMock` is a subclass of `Mock` that provides additional 
convenience methods for mocking special methods, such as the ones used for 
context managers or magic methods like `__getitem__`, `__setitem__`, and 
`__iter__`. While `Mock` can also be used for this purpose, `MagicMock` makes it 
easier to work with these special methods.

Example of using `MagicMock`:

```python
from unittest.mock import MagicMock

def test_my_function():
    magic_mock_object = MagicMock()
    magic_mock_object.__getitem__.side_effect = lambda x: x * 2
    
    result = my_function(magic_mock_object)
    
    magic_mock_object.__getitem__.assert_called_with(5)
    assert result == 10
```

In summary:

- `patch` is used to temporarily replace objects (e.g., functions or classes) 
during test execution.
- `Mock` is a class for creating flexible mock objects that can replace real 
objects in tests.
- `MagicMock` is a subclass of `Mock` that provides additional convenience for 
mocking special methods.


"""
