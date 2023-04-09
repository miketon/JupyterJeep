import uuid


class Entity:
    def __init__(self, id=None, name="empty"):
        """Simple Entity class with a unique identifier for each entity"""
        # @note 🧠 : Id is how components are identified, NEEDS to be FIRST argument
        self.id = id or self.generate_unique_id()
        self.name = name

    @staticmethod
    def generate_unique_id():
        return str(uuid.uuid4())
