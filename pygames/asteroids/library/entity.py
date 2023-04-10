import uuid


class Entity:
    @staticmethod
    def generate_unique_id() -> str:
        return str(uuid.uuid4())

    def __init__(self, id: str = "", name: str = ""):
        """Simple Entity class with a unique identifier for each entity"""
        # @note 🧠 : Id is how components are identified, NEEDS to be FIRST argument
        self.id: str = id or self.generate_unique_id()
        self.name: str = name

    def __repr__(self):
        """ 
        The `__repr__` method is a special method in Python classes that is used to 
        define a human-readable string representation of an object. 
        When you use the `print()` function or enter an object in an interactive shell, 
        Python will call the object's `__repr__` method to get a string representation 
        that can be displayed.
        """
        return f"<Entity id={self.id} name={self.name}>"

'''
    # @note 🧠 : Actually we do want to compare based on memory address, to 
    ensure no unnecessary duplicate entities are created -- COMMENTING OUT
    def __eq__(self, other):
        """ 
        `__eq__` is the method to compare the entities based on their attributes 
        rather than their memory addresses

        When you compare Entity objects based on their memory addresses, you are 
        checking if those two objects are the exact same object stored in the 
        same location in the computer's memory. Memory addresses are unique for 
        each object, so if two objects have different memory addresses, they are 
        considered different.

        Comparing Entity objects based on their content means you are checking 
        if the information stored in those objects is the same or not. In this 
        case, you look at the attributes of the objects and see if they have 
        the same values. For example, if you compare two Entity objects based 
        on their name attribute, they would be considered equal if they have t
        he same name, even if they are stored at different memory addresses.

        Here's an analogy:

        - Comparing memory addresses is like checking if two people are the same 
        person. Even if they look the same and have the same name, they are 
        considered different because they are two separate individuals.
        - Comparing content is like checking if two people have the same name. 
        In this case, we don't care if they are different individuals; we only 
        care that their names match.

        In the context of your test, you want to compare the Entity objects 
        based on their content (e.g., their names) rather than their memory addresses because you are interested in whether the objects have the same information, not if they are the exact same objects in memory.

        ---------- Explained like I am 5 ----------

        Imagine you have two identical toy cars, one on your table and another 
        in your toy box. They look the same, but they are in different places. 
        In this situation, the test is checking if the toy car on the table is 
        the same as the one in the toy box. Since they are in different places, 
        the test says they are not the same.

        In the code, the test is checking if two `Entity` objects are the same 
        but is looking at their memory addresses 
        (their places in the computer's memory) instead of their actual content. 

        The memory addresses are different, so the test thinks the `Entity` 
        objects are not the same even if they have the same content.
        
        To fix this, we need to tell the test to compare the `Entity` objects 
        based on their content (like their names) instead of their memory 
        addresses. That way, if two `Entity` objects have the same content, 
        the test will say they are the same.
        """
        if isinstance(other, Entity):
            return self.id == other.id
        return False
'''
