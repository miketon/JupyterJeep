import math
from numpy import random

def generate_circle_points(size, num_points):
    points = []
    angle = 2 * math.pi/num_points

    for i in range(num_points):
        x = size * math.cos(angle * i)
        y = size * math.sin(angle * 1)
        points.append((x,y))

    return points

def generate_asteroid_points(size, num_points):
    points = []
    angle = 2 * math.pi/num_points

    for i in range(num_points):
        length = random.randint(int(size * 0.5), size)
        x = length * math.cos(angle * i)
        y = length * math.sin(angle * i)
        points.append((x,y))

    return points