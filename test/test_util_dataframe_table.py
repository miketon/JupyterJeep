import unittest
import torch
import pandas as pd
import numpy as np
from IPython.display import display

from utils.dataframe.util_dataframe_table import UtilDataFrameTable


class TestUtilDataFrameTable(unittest.TestCase):
    def test_show_table_invalid(self):
        """
        Test show_table() handling invalid input
        """
        # make sure ValueError is raised with None input
        with self.assertRaises(ValueError):
            UtilDataFrameTable.show_table(None)  # type: ignore
        with self.assertRaises(ValueError):
            UtilDataFrameTable.show_table(np.arange(1, 10))  # type: ignore
        with self.assertRaises(ValueError):
            # empty tensor
            UtilDataFrameTable.show_table(torch.tensor([]))
        with self.assertRaises(ValueError):
            # 1d tensor
            UtilDataFrameTable.show_table(torch.tensor([1]))
        # make sure no error is raised with a valid input
        try:
            # empyt DataFrame - default to 0x0 and are valid
            UtilDataFrameTable.show_table(pd.DataFrame([]))
        except ValueError:
            self.fail("show_table() raised ValueError unexpectedly!")
        try:
            tensor_2d = torch.tensor([[1, 2, 3], [4, 5, 6]])
            UtilDataFrameTable.show_table(tensor_2d)
        except ValueError:
            self.fail("show_table() raised ValueError unexpectedly!")

    def test_show_table_tensor(self):
        """
        Test show_table() handling a Tensor input
        """
        tensor = torch.tensor([[1, 2, 3], [4, 5, 6]])
        on_complete = UtilDataFrameTable.show_table(tensor)
        self.assertTrue(on_complete)

    def test_show_table_df(self):
        """
        Test show_table() handling DataFrame input
        """
        df = pd.DataFrame([[1, 2, 3], [4, 5, 6]])
        on_complete = UtilDataFrameTable.show_table(df)
        self.assertTrue(on_complete)

    def test_show_table_func_applied(self):
        """
        Test show_table() handling a function applied to input
        """
        tensor = torch.tensor([[1, 2, 3], [4, 5, 6]])
        on_complete = UtilDataFrameTable.show_table(tensor, func=lambda x: x + 1)
        self.assertTrue(on_complete)

    def test_array_to_table_root(self):
        """
        Test array_to_table() handling a 1D array that square roots cleanly
        """
        array = np.arange(1, 10)  # 9
        table = UtilDataFrameTable.array_to_table(array)
        np.testing.assert_array_equal(
            table, np.array([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
        )

    def test_array_to_table_even(self):
        """
        Test array_to_table() handling a 1d array that is evenly divisible
        """
        array = np.array([1, 2, 3, 4, 5, 6, 7, 8])
        table = UtilDataFrameTable.array_to_table(array)
        pad = UtilDataFrameTable.PADDING_DEFAULT
        np.testing.assert_array_equal(
            table, np.array([[1, 2, 3], [4, 5, 6], [7, 8, pad]])
        )

    def test_array_to_table_odd(self):
        """
        Test array_to_table() handling a 1d array that is non-evenly divisible
        """
        array = np.array([1, 2, 3, 4, 5, 6, 7])
        table = UtilDataFrameTable.array_to_table(array)
        pad = UtilDataFrameTable.PADDING_DEFAULT
        np.testing.assert_array_equal(
            table, np.array([[1, 2, 3], [4, 5, 6], [7, pad, pad]])
        )

    def test_array_to_table_large(self):
        """
        Test array_to_table() handling a 1d array that is larger
        """
        array = np.arange(1, 64)  # 63
        length = len(array)
        """
        The main differences between `np.resize()` and `reshape()` are:
            1. `np.resize()` can change the number of elements in the array and 
            fill the extra elements by repeating the original array, while 
            `reshape()` requires the new shape to have the same number of 
            elements as the original array.
            2. `np.resize()` returns a new array with the specified shape, and 
            the original array remains unchanged. In contrast, `reshape()` 
            returns a new view on the original array, and they share the same 
            data.
        """
        expected_table = np.resize(array, (8, 8))
        # Fill in entry with PADDING_DEFAULT instead of wrapping around
        expected_table[-1][-1] = UtilDataFrameTable.PADDING_DEFAULT
        table = UtilDataFrameTable.array_to_table(array)
        np.testing.assert_array_equal(table, expected_table)

    def test_array_to_table_chars(self):
        """
        Test array_to_table() handling a 1d array of characters
        """
        array = np.array(["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"])  # 10
        table = UtilDataFrameTable.array_to_table(array)
        pad = UtilDataFrameTable.PADDING_DEFAULT
        np.testing.assert_array_equal(
            table,
            np.array(
                [["a", "b", "c", "d"], ["e", "f", "g", "h"], ["i", "j", pad, pad]]
            ),
        )

    def test_array_to_table_char_mix(self):
        """
        Test array_to_table() handling a 1d array of mixed characters and numbers
        """
        array = np.array([0, "a", 1, "b", 2, "c", 3, "d", 4, "e", 5])

        table = UtilDataFrameTable.array_to_table(array)
        """ 
        In this case, it's because the array contains both numbers and 
        characters, and they are stored as Unicode strings.
            - `<U21` means "Unicode string of max length 21". 
            - `dtype='<U21'` - This is the data type of the elements in the array 
        """
        pad = UtilDataFrameTable.PADDING_DEFAULT
        np.testing.assert_array_equal(
            table,
            np.array(
                [["0", "a", "1", "b"], ["2", "c", "3", "d"], ["4", "e", "5", pad]]
            ),
        )

    def test_array_to_table_padding_value(self):
        """
        Test array_to_table() handling a 1d array of mixed characters and numbers
        """
        array = np.array([0, "a", 1, "b", 2, "c", 3, "d", 4, "e", 5])
        # valid padding value
        pad = "911"
        table = UtilDataFrameTable.array_to_table(array, pad_value=pad)
        np.testing.assert_array_equal(
            table,
            np.array(
                [["0", "a", "1", "b"], ["2", "c", "3", "d"], ["4", "e", "5", pad]]
            ),
        )
        # invalid padding value, we should get a warning and the table should be DEFAULT_PADDING
        pad = "*"
        table = UtilDataFrameTable.array_to_table(array, pad_value=pad)
        np.testing.assert_array_equal(
            table,
            np.array(
                [
                    ["0", "a", "1", "b"],
                    ["2", "c", "3", "d"],
                    ["4", "e", "5", UtilDataFrameTable.PADDING_DEFAULT],
                ]
            ),
        )


if __name__ == "__main__":
    unittest.main()
