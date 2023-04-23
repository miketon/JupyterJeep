import math
import numpy as np
import pandas as pd
import torch
from typing import Optional, Callable, Union
from IPython.display import display
import logging


class UtilDataFrameTable:
    """
    Utility class to generate and display a DataFrame table
    """

    PADDING_DEFAULT: int = 0

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
        if not isinstance(data, (torch.Tensor, pd.DataFrame)):
            raise ValueError("data must be a torch.Tensor or pd.DataFrame")
        elif len(data.shape) < 2:
            raise ValueError("data.shape has to be at least 2D")

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
    def array_to_table(data: np.ndarray, pad_value="") -> np.ndarray:
        """
        Convert a 1D array to balanced 2D table
        :param data: 1D Numpy array
        :return: 2D Numpy array padded to be balanced
        """
        # Calculate the total number of elements in the input array
        total_elements = len(data)
        # find the closest lower square
        data_sq_x = data_sq_y = int(math.ceil(math.sqrt(total_elements)))
        mod = total_elements % data_sq_x
        if mod != 0:
            # default pad value is 0
            if not UtilDataFrameTable._is_convertible_to_int(pad_value):
                logging.warning(
                    f"pad_value {pad_value} is not convertible to int : setting to default {UtilDataFrameTable.PADDING_DEFAULT}"
                )
                pad_value = UtilDataFrameTable.PADDING_DEFAULT
            # Calculate the next perfect square
            # Pad the input array with zeros
            padding = (data_sq_x * data_sq_y) - total_elements
            if padding > data_sq_x:
                padding = padding - data_sq_x
                data_sq_x = data_sq_x - 1

            data = np.pad(
                data, (0, np.abs(padding)), "constant", constant_values=pad_value
            )

        # reshape the padded array to the desired dimension
        table = data.reshape(data_sq_x, data_sq_y)
        return table

    @staticmethod
    def _is_convertible_to_int(x):
        try:
            int(x)
            return True
        except ValueError:
            return False
