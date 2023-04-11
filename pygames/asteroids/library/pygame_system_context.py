import os
import sys
import pygame
from pygame.event import Event
from pygame.math import Vector2
from typing import NamedTuple, List, Dict, Any


class SystemContext(NamedTuple):
    screen: pygame.Surface
    clock: pygame.time.Clock


# System Context will be init on awake
class RenderContext(NamedTuple):
    WIDTH: int
    HEIGHT: int
    BACKGROUND_COLOR: tuple
    FPS: int


class AssetContext(NamedTuple):
    ASSET_FOLDER_PATH: str


class PygameSystemContext:
    ASSET = AssetContext(ASSET_FOLDER_PATH=os.path.join("assets", "images"))

    # Statically define CONTEXT CONSTANTS
    RENDER = RenderContext(
        640,  # RESOLUTION_WIDTH
        480,  # RESOLUTION_HEIGHT
        (64, 64, 0),  # BACKGROUND_COLOR
        30,  # FPS
    )

    @staticmethod
    def on_awake() -> SystemContext:
        """
        Initialize pygame
        - SYS_SCREEN
        - SYS_CLOCK
        """
        # Initialize pygame
        pygame.init()
        # Initialize the screen
        screen = pygame.display.set_mode(
            # @audit-ok 💨 : Use `HWSURFACE` and `DOUBLEBUF` for better performance
            (
                PygameSystemContext.RENDER.WIDTH,
                PygameSystemContext.RENDER.HEIGHT,
            ),
            pygame.HWSURFACE | pygame.DOUBLEBUF,
        )
        # Initialize the clock
        # @audit-ok 💨 : Cap the frame rate else loop will run at system allows and
        # # will lead to :
        # - high CPU usage
        # - inconsistent performance across different devices.
        clock = pygame.time.Clock()

        # Set the title and icon (optional)
        pygame.display.set_caption("Asteroids_ECS_Edition")
        icon = pygame.image.load(
            os.path.join(PygameSystemContext.ASSET.ASSET_FOLDER_PATH, "app_icon.jpg")
        ).convert_alpha()
        pygame.display.set_icon(icon)

        return SystemContext(screen, clock)

    @staticmethod
    def on_exit() -> None:
        """Handle exit event"""
        pygame.quit()
        sys.exit()
