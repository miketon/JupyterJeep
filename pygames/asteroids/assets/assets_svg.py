import pygame
import os
from .generate_svg import generate_png

SVG_FOLDER_PATH = os.path.join(os.path.dirname(__file__), 'svg')

def svg_ufo() -> pygame.Surface:
    file_path = os.path.join(SVG_FOLDER_PATH, 'ufo.svg')
    return generate_png(file_path)