import math
import numpy as np
import pandas as pd
import torch
from typing import Optional, Callable, Union
from IPython.display import display


class UtilDataFrameTable:
    """
    Utility class to generate and display a DataFrame table
    """

    @staticmethod
    def show_table(
        data: Union[torch.Tensor, pd.DataFrame],
        func: Optional[Callable] = None,
        label: Optional[str] = None,
    ) -> bool:
        """
        Generates a DataFrame table and display() it given a tensor and map function
        :param data: torch.Tensor or pd.DataFrame (to visualize tables that have strings)
        """

        if label is not None:
            print(f"--[{label}]--")

        if isinstance(data, torch.Tensor):
            df = pd.DataFrame(data.numpy())
        else:
            df = data

        if func is None:
            display(df)
        else:
            df_result = df.applymap(lambda x: func(int(x)))
            display(pd.concat([df, df_result]).sort_index(kind="mergesort"))
        # has displayed table successfully
        return True

    @staticmethod
    def array_to_table(data: np.ndarray) -> np.ndarray:
        """
        Convert a 1D array to balanced 2D table
        :param data: 1D Numpy array
        :return: 2D Numpy array padded to be balanced
        """
        # Calculate the total number of elements in the input array
        total_elements = len(data)
        # find the closest lower square
        data_sq_root = int(math.sqrt(total_elements))
        if total_elements % data_sq_root != 0:
            # Calculate the next perfect square
            data_sq_root += 1
            # Pad the input array with zeros to make it a perfect square
            padding = (data_sq_root ** 2) - total_elements
            data = np.pad(data, (0, np.abs(padding)), "constant", constant_values=0)

        # reshape the padded array to the desired dimension
        table = data.reshape(data_sq_root, -1)
        return table
