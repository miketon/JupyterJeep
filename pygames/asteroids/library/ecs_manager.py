from abc import ABC, abstractmethod
from typing import Any, Dict, List, Optional, Type, TypeVar
from collections import defaultdict
from .pubsub_event_manager import PubSubEventManager
from .entity import Entity
from .environment_flag import EnvironmentFlag

# type variable `T`. This allows you to define a function that works with any
# type, while still providing type hints for better code readability and
# editor support.
T = TypeVar("T")


# Pythonic to name suffix ABC for abstract base classes, instead of `Interface``
class ComponentManagerABC(ABC):
    """Interface for a Component Manager Class"""

    @abstractmethod
    def add_component(self, entity, component: Any) -> None:
        """Add a component to an entity"""

    @abstractmethod
    def remove_component(self, entity, component_type: Type) -> None:
        """Remove a component of a specific type from an entity"""

    #  The `-> T` part indicates that the function returns a value of type `T`
    # - Since `T` is a generic type variable, it means that the return type of
    # this function is the same as the type passed to the `component_type`
    # parameter
    @abstractmethod
    def get_component(self, entity, component_type: Type[T]) -> T:
        """Get a component of a specific type from an entity"""

    @abstractmethod
    def gather_entities_with_all_components(
        self, component_types: List[Type]
    ) -> List[Entity]:
        """Gather a list of entities that have ALL of the specified COMPONENTS"""


# Implement a simple ComponentManager class that manages a dictionary of components
class ComponentManager(ComponentManagerABC):
    # Define constants for the event names to publish and subscribe to
    ADD_COMPONENT = "add_component"
    REMOVE_COMPONENT = "remove_component"

    def __init__(self, event_manager: PubSubEventManager):
        self.envFlag = EnvironmentFlag()
        # With a `defaultdict`, you can directly add a component without first
        # checking if the `entity.id` exists in `self.components`.
        # The `defaultdict` is a specialized version of the built-in `dict` that
        # provides a default value for a key if the key is not found in the
        # dictionary. In this case, if a key is not found, it will automatically
        # create a new empty dictionary as its value.
        # - simplifies the code for `add_component`
        self.components: Dict[str, Dict[Type, Any]] = defaultdict(dict)
        self.event_manager = event_manager
        # - `self.entities` is a dictionary that maps entity IDs to entity
        # instances. This is used to quickly look up an entity by its ID.
        self.entities = {}

    def add_component(self, entity, component: Any) -> None:
        # - defaultdict will automatically create a new dictionary for the entity
        # if it does not exist : greatly simplifies the code
        # - check if component already exists
        # - When checking if an object is an instance of a class, it's recommended
        # to use the `isinstance` function instead of directly comparing types
        # This allows you to handle cases where `component` is an instance of a
        # subclass of the expected component type
        if any(
            isinstance(c, type(component)) for c in self.components[entity.id].values()
        ):
            raise ValueError(
                f"[ERROR] tried to --add_component-- [{type(component).__name__}] that already exists on entity [{entity.name}]"
            )
        self.components[entity.id][type(component)] = component
        # also add to the self.entities dictionary cache with the entity object
        self.entities[entity.id] = entity
        self.event_manager.publish(
            ComponentManager.ADD_COMPONENT, entity=entity, component=component
        )

    def remove_component(self, entity, component_type: Type) -> None:
        try:
            del self.components[entity.id][component_type]
            self.event_manager.publish(
                ComponentManager.REMOVE_COMPONENT,
                entity=entity,
                component_type=component_type,
            )
        except KeyError:
            raise ValueError(
                f"[ERROR] tried to --remove_component-- [{component_type.__name__}] that does not exist on entity [{entity.name}]"
            )

    def get_component(self, entity, component_type: Type[T]) -> T:
        # Because `None` is not compatible with the expected return type `T`
        # - To fix this issue, you can use the `in` keyword to check if the
        # `component_type` key exists in the `self.components[entity.id]`
        # dictionary before returning its value
        if (
            entity.id in self.components
            and component_type in self.components[entity.id]
        ):
            return self.components[entity.id][component_type]
        else:
            raise ValueError(
                f"[ERROR] tried to --get_component-- [{component_type.__name__}] that does not exist on entity [{entity.name}]"
            )

    def gather_entities_with_all_components(
        self, component_types: List[Type]
    ) -> List[Entity]:
        # early exit with None if no component types passed in
        if not component_types:
            return []
        # @audit 💨 : Convert the list of component types to a set for faster lookup
        # - in O(1) time instead of O(n) time where n is # of component types
        # - order of component types passed in does not matter, any permutation
        # will give the same result
        component_set = set(component_types)
        matching_entities = []
        for entity_id, components in self.components.items():
            if component_set.issubset(set(components.keys())):
                # get the cached entity instance from the `self.entities` dictionary
                matching_entity = self.entities[entity_id]
                matching_entities.append(matching_entity)
        
        # @audit 💨 : Only print debug info if not in production
        if not self.envFlag.is_production :
            ComponentManager.entities_gathered_debug_print(
                matching_entities, component_types, self.entities_count()
            )
        return matching_entities

    def entities_count(self):
        """Returns the number of entities in the game"""
        return len(self.entities)

    @staticmethod
    def entities_gathered_debug_print(
        gathered_entities: List[Entity],
        list_component: List[Type],
        total_entities_count: int,
    ):
        """Debug print the list of entities"""
        component_names = [component.__name__ for component in list_component]
        # @todo : using split workaround to get entity name from Entity object
        # Fix this later
        gathered_entities_names = [str(entity.name).split('_')[0] for entity in gathered_entities]

        print(
            f"[entities_debug_print]: [{len(gathered_entities)}]/[{total_entities_count}] with components [{len(component_names)}] {component_names} gathered [{len(gathered_entities)}] {gathered_entities_names}"
        )