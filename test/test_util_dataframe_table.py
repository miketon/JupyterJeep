import unittest
import torch
import pandas as pd
import numpy as np
from IPython.display import display, DisplayHandle

from utils.dataframe.util_dataframe_table import UtilDataFrameTable


class TestUtilDataFrameTable(unittest.TestCase):
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
        array = np.arange(1,10) # 9
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
        np.testing.assert_array_equal(
            table, np.array([[1, 2, 3, 4], 
                             [5, 6, 7, 8]])
        )

    def test_array_to_table_odd(self):
        """
        Test array_to_table() handling a 1d array that is non-evenly divisible
        """
        array = np.array([1, 2, 3, 4, 5, 6, 7])
        table = UtilDataFrameTable.array_to_table(array)
        np.testing.assert_array_equal(
            table, np.array([[1, 2, 3], 
                             [4, 5, 6], 
                             [7, 0, 0]])
        )

    def test_array_to_table_char(self):
        """
        Test array_to_table() handling a 1d array of characters
        """ 
        array = np.array(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']) # 10
        table = UtilDataFrameTable.array_to_table(array)
        np.testing.assert_array_equal(
            table, np.array([['a', 'b', 'c', 'd'], 
                             ['e','f', 'g', 'h'], 
                             ['i', 'j', '0', '0'], 
                             ['0', '0', '0', '0']])
        )
    

    def test_array_to_table_char_mix(self):
        """
        Test array_to_table() handling a 1d array of mixed characters and numbers
        """
        


if __name__ == "__main__":
    unittest.main()
