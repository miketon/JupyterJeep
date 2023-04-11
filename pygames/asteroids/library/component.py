from collections import namedtuple
from abc import ABC, abstractmethod


class ComponentABC(ABC):
    def __init__(self, **kwargs):
        pass

    def __repr__(self) -> str:
        return self.__class__.__name__
