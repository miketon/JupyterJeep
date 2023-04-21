import unittest
import torch
from IPython.display import display, DisplayHandle

from utils.dataframe.util_dataframe_table import UtilDataFrameTable


class TestUtilDataFrameTable(unittest.TestCase):
    def test_show_table_tensor(self):
        """
        Test show_table() with a tensor
        """
        tensor = torch.tensor([[1, 2, 3], [4, 5, 6]])
        has_displayed_table = UtilDataFrameTable.show_table(tensor)
        self.assertTrue(has_displayed_table)


if __name__ == "__main__":
    unittest.main()
