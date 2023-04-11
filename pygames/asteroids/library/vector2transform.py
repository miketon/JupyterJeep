import math
from pygame.math import Vector2
from typing import List

class Vector2Transform:
    """
    Helper class for performing 2D vector transformations
    """

    @staticmethod
    def rotate_vector(vector: Vector2, radian_angle: float) -> Vector2:
        """Rotate a vector by a given radian angle, center is assumed to be (0,0)"""
        return vector.rotate_rad(radian_angle)

    @staticmethod
    def angle_from_vector(vector: Vector2) -> float:
        """Get the angle of a vector in radians"""
        return math.atan2(vector.y, vector.x)

    @staticmethod
    def rotate_vectors(points: List[Vector2], angle: float) -> List[Vector2]:
        """Rotate a list of points by a given angle, center is assumed to be (0,0)"""
        angle_rad = math.radians(angle)
        rotated_points: List[Vector2] = []
        for point in points:
            rotated_vector = Vector2Transform.rotate_vector(point, angle_rad)
            rotated_points.append(rotated_vector)
        return rotated_points
