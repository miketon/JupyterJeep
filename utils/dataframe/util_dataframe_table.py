import math
import numpy as np
import pandas as pd
import torch
from typing import Optional, Callable, Union
from IPython.display import display, DisplayHandle


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
        Convert a 1D array to a padded (balanced) 2D table
        :param data: 1D Numpy array
        :return: 2D Numpy array padded to be balanced
        """
        # Calculate the total number of elements in the input array
        total_elements = len(data)
        # find the closes lower square
        lower_square = int(math.sqrt(total_elements)) ** 2

        # calculate the square root of the closest lower square
        square_root = int(math.sqrt(lower_square))

        # caculate the required padding length
        padding_length = (square_root + 1) * square_root - total_elements

        # pad the Numpy array with the next consequitive integers
        padded_array = np.pad(
            data, (0, padding_length), mode="constant", constant_values=(np.nan,)
        )

        # reshape the padded array to the desired dimension
        table = padded_array.reshape(square_root, square_root + 1)
        return table
