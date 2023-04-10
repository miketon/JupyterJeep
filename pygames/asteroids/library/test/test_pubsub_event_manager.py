import unittest
from ..pubsub_event_manager import PubSubEventManager

UNIT_TEST_EVENT_NAME_1 = "contra_complete_1_life"
UNIT_TEST_EVENT_NAME_2 = "contra_complete_konami_code"


class TestPubSubEventManager(unittest.TestCase):
    def test_subscribe(self):
        # test case for subscribing to an event
        def callback(**kwargs):
            pass

        event_manager = PubSubEventManager()
        event_manager.subscribe(UNIT_TEST_EVENT_NAME_1, callback)

        self.assertIn(UNIT_TEST_EVENT_NAME_1, event_manager.listeners)
        self.assertIn(callback, event_manager.listeners[UNIT_TEST_EVENT_NAME_1])

    def test_publish(self):
        # test case for publishing an event
        self.event_triggered = False

        def callback(**kwargs):
            self.event_triggered = True
            self.assertEqual(kwargs["data"], "test_data")

        event_manager = PubSubEventManager()
        event_manager.subscribe(UNIT_TEST_EVENT_NAME_2, callback)
        event_manager.publish(UNIT_TEST_EVENT_NAME_2, data="test_data")

        self.assertTrue(self.event_triggered)


if __name__ == "__main__":
    unittest.main()
