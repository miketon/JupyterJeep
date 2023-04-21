import unittest
from pygames.asteroids.library.ecs_manager import ComponentManager, ComponentManagerABC
from pygames.asteroids.library.entity import Entity
from pygames.asteroids.library.pubsub_event_manager import PubSubEventManager
from typing import List
from abc import ABC


class SleepComponent(ABC):
    pass


class DietComponent(ABC):
    pass


class ExerciseComponent(ABC):
    pass


class TestHelper:
    @staticmethod
    def setup_component_manager():
        """Helper method to setup a component manager for testing"""
        event_manager = PubSubEventManager()
        component_manager = ComponentManager(event_manager)
        return component_manager


class TestComponentManager(unittest.TestCase):
    def test_add_component(self):
        component_manager = TestHelper.setup_component_manager()
        entity = Entity("JigglyPuff")
        component = SleepComponent()

        component_manager.add_component(entity, component)
        self.assertIn(entity.id, component_manager.components)
        self.assertIn(type(component), component_manager.components[entity.id])

    def test_add_component_duplicate(self):
        component_manager = TestHelper.setup_component_manager()
        entity = Entity("Kirby")
        component = DietComponent()

        component_manager.add_component(entity, component)
        # verify that adding the same component again raises an error
        with self.assertRaises(ValueError):
            component_manager.add_component(entity, component)

    def test_remove_component(self):
        component_manager = TestHelper.setup_component_manager()
        entity = Entity("Little Mac")
        component = ExerciseComponent()

        component_manager.add_component(entity, component)
        self.assertIn(entity.id, component_manager.components)
        # verify that the component is in the components dictionary
        self.assertIn(type(component), component_manager.components[entity.id])
        component_manager.remove_component(entity, type(component))
        # verify that the component is no longer in the components dictionary
        # after removal
        self.assertNotIn(type(component), component_manager.components[entity.id])

    def test_remove_component_nonexistent(self):
        component_manager = TestHelper.setup_component_manager()
        entity = Entity("JigglyPuff")
        component = SleepComponent()

        # verify that removing a component that does not exist raises an error
        with self.assertRaises(ValueError):
            component_manager.remove_component(entity, type(component))

    def test_get_component(self):
        component_manager = TestHelper.setup_component_manager()
        entity = Entity("Kirby")
        component = DietComponent()

        component_manager.add_component(entity, component)
        self.assertEqual(
            component, component_manager.get_component(entity, type(component))
        )

    def test_get_component_nonexistent(self):
        component_manager = TestHelper.setup_component_manager()
        entity = Entity("JigglyPuff")
        component = SleepComponent()

        # verify that getting a component that does not exist raises an error
        with self.assertRaises(ValueError):
            component_manager.get_component(entity, type(component))


class TestComponentManager_Gather_Entities(unittest.TestCase):
    def test_gather_entities_empty_entities(self):
        component_manager = TestHelper.setup_component_manager()
        # empty Entity list expected to return empty match list
        expected_result: List[Entity] = []
        result = component_manager.gather_entities_with_components(
            [SleepComponent, DietComponent]
        )
        assert result == expected_result, f"Expected {expected_result}, got {result}"

    def test_gather_entities_empty_components(self):
        entity1 = Entity("JigglyPuff")
        entity2 = Entity("Kirby")
        component_manager = TestHelper.setup_component_manager()
        # empty component input expected to return empty Entity list
        expected_result: List[Entity] = []
        result = component_manager.gather_entities_with_components([])
        assert result == expected_result, f"Expected {expected_result}, got {result}"

    def test_gather_entities_single_component_match(self):
        entity1 = Entity("JigglyPuff")
        entity2 = Entity("Kirby")
        component_manager = TestHelper.setup_component_manager()
        component_manager.add_component(entity1, SleepComponent())
        component_manager.add_component(entity2, DietComponent())
        # expecting Sleepy Kirby to match
        expected_results: List[Entity] = [entity1]
        result = component_manager.gather_entities_with_components([SleepComponent])
        assert result == expected_results, f"Expected {expected_results}, got {result}"

    def test_gather_entities_single_component_no_match(self):
        entity1 = Entity("JigglyPuff")
        entity2 = Entity("Kirby")
        entity3 = Entity("Little Mac")

        component_manager = TestHelper.setup_component_manager()
        component_manager.add_component(entity1, SleepComponent())
        component_manager.add_component(entity2, DietComponent())
        # expecting no entity, since Little Mac is not exercising
        expected_result: List[Entity] = []
        result = component_manager.gather_entities_with_components(
            [ExerciseComponent]
        )
        assert result == expected_result, f"Expected {expected_result}, got {result}"

    def test_gather_entities_multi_component_match(self):
        entity1 = Entity("JigglyPuff")
        entity2 = Entity("Kirby")
        entity3 = Entity("Little Mac")

        component_manager = TestHelper.setup_component_manager()
        component_manager.add_component(entity1, SleepComponent())
        component_manager.add_component(entity2, DietComponent())
        component_manager.add_component(entity3, ExerciseComponent())
        component_manager.add_component(entity3, DietComponent())
        # expecting Little Mac to match
        expected_results: List[Entity] = [entity3]
        result = component_manager.gather_entities_with_components(
            [ExerciseComponent, DietComponent]
        )
        assert result == expected_results, f"Expected {expected_results}, got {result}"

    def test_gather_entities_multi_component_no_match(self):
        expected_result: List[Entity] = []
        entity1 = Entity("JigglyPuff")
        entity2 = Entity("Kirby")

        component_manager = TestHelper.setup_component_manager()
        component_manager.add_component(entity1, SleepComponent())
        component_manager.add_component(entity1, ExerciseComponent())
        component_manager.add_component(entity2, DietComponent())
        # expecting no entity has BOTH SleepComponent AND DietComponent
        result = component_manager.gather_entities_with_components(
            [SleepComponent, DietComponent]
        )
        assert result == expected_result, f"Expected {expected_result}, got {result}"
