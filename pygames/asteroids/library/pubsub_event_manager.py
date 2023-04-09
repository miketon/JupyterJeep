from abc import ABC, abstractmethod


class PubSubEventManagerABC(ABC):
    """Interface for a Publish and Subcribe Class"""

    @abstractmethod
    def subscribe(self, event_name, callback) -> None:
        """subscribe to [event] name and pass [callback] method"""
        pass

    @abstractmethod
    def publish(self, event_name, **kwargs) -> None:
        """publish [event] name"""
        pass


class PubSubEventManager(PubSubEventManagerABC):
    def __init__(self):
        self.listeners = {}

    def subscribe(self, event_name, callback) -> None:
        super().subscribe(event_name, callback)
        if event_name not in self.listeners:
            # print(f'[PubSubEventManager] : Subscribed to "{event_name}"')
            self.listeners[event_name] = []
        self.listeners[event_name].append(callback)

    def publish(self, event_name, **kwargs) -> None:
        super().publish(event_name, **kwargs)
        if event_name in self.listeners:
            # print(f'[PubSubEventManager] : Published "{event_name}"')
            for callback in self.listeners[event_name]:
                callback(**kwargs)
