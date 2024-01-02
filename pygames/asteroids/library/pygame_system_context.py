import os
import sys
import pygame
from pygame.event import Event
from pygame.math import Vector2
from typing import NamedTuple, List, Dict, Any


# GameConfig will be init on awake
class GameConfig(NamedTuple):
    WIDTH: int = 640
    HEIGHT: int = 480
    BACKGROUND_COLOR = (64, 64, 0)
    FPS: int = 30
    ASSET_FOLDER_PATH: str = os.path.join("assets", "images")
    GAME_NAME: str = "Asteroids"
    GAME_VERSION: str = "ECS Edition"


class SystemContext(NamedTuple):
    screen: pygame.Surface
    clock: pygame.time.Clock


class PygameSystemContext:
    CONFIG = GameConfig()
    # load asset on initialization, loading on_awake will cause delay
    ICON = pygame.image.load(os.path.join(CONFIG.ASSET_FOLDER_PATH, "app_icon.jpg"))

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
                PygameSystemContext.CONFIG.WIDTH,
                PygameSystemContext.CONFIG.HEIGHT,
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
        pygame.display.set_caption(
            f"{PygameSystemContext.CONFIG.GAME_NAME} {PygameSystemContext.CONFIG.GAME_VERSION}"
        )
        pygame.display.set_icon(PygameSystemContext.ICON)

        return SystemContext(screen, clock)

    @staticmethod
    def on_exit() -> None:
        """Handle exit event"""
        pygame.quit()
        sys.exit()
