import unittest
from pubsub_event_manager import PubSubEventManager, PubSubEventManagerABC

UNIT_TEST_EVENT_NAME = 'contra_complete_1_life'

class TestPubSubEventManager(unittest.TestCase):
    def test_subscribe(self):
        #test case for subscribing to an event
        def callback(**kwargs):
            pass

        event_manager = PubSubEventManager()
        event_manager.subscribe(UNIT_TEST_EVENT_NAME, callback)

        self.assertIn(UNIT_TEST_EVENT_NAME, event_manager.listeners)
        self.assertIn(callback, event_manager.listeners[UNIT_TEST_EVENT_NAME])

"""
    def test_publish(self):
        #test case for publishing an event
"""

if __name__ == '__main__':
    unittest.main()