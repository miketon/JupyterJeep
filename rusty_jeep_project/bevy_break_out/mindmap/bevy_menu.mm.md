---
markmap:
   colorFreezeLevel: 2
   maxWidth: 500
---

# Menu

## Project

### | SETTINGS |

#### `settings.json`

- -- path to project settings --
  - 🍅 cargo.toml 🍅

    - ```json
        {
          "rust-analyzer.linkedProjects": [
              "./Cargo.toml"
          ]
        }
      ```


### | CONFIG |

#### `launch.json`

- -- debug -- **(⇧⌘D)**

  - ```json
      {
        "version": "0.2.0",
        "configurations": [ ... ]
      }
    ```

    - // configure to add to **RUN and DEBUG** tab **dropdown**

      - ```json
          {
              "type": "lldb",
              "request": "launch",
              "name": "Debug example 'bevy_ecs_tilemap'",
              "cargo": {
                  "args": [
                      "build",
                      "--example=bevy_ecs_tilemap",
                      "--package=bevy_break_out"
                  ],
                  "filter": {
                      "name": "bevy_ecs_tilemap",
                      "kind": "example"
                  }
              },
              "args": [],
              "cwd": "${workspaceFolder}"
          },
        ```

### | COMMANDS |

#### `tasks.json`

- -- run -- **(⌘⇧B)**

  - ```json
      {
        "version": "2.0.0",
        "tasks": [ ... ]
      }
    ```

    - // configure to add **RUN** keyboard **shortcut**

      - ```json
          {
            "type": "cargo",
            "command": "run",
            "problemMatcher": [
              "$rustc"
            ],
            "label": "rust: cargo run"
          },
        ```

## Imports

### 🔌 = ==[ Plugin ]==

- 🍅 `Cargo.toml` 🍅
  - [package]
    - name = "bevy_break_out"
    - version = "0.1.0"
    - edition = "2021"
  - [dependencies]
    - bevy = {version = "0.10", features = ["mp3"]}
    - bevy_editor_pls = "0.4"
    - bevy-inspector-egui = "0.18"
  - [profile.dev]
    - opt-level = 1
      - // Enable a small amount of optimization in debug mode
  - [profile.dev.package."*"]
    - opt-level = 3
      - // Enable high optimization for dependencies
       (including Bevy)
      - // but not for OUR code

### ➕ = ==[ Mod ]==

#### -- source --

- 💾 src/
  - | FOLDERS |
    - 🔝 bundles/
      - `mod.rs`
      - @audit : why not bundles.rs ??
    - 🔝 configs/
      - `mod.rs`
      - @audit : why not configs.rs ??
  - | FILES |
    - app_state.rs
    - game.rs
    - menu.rs
    - splash.rs

#### -- import --

- 💾 src/
  - `main.rs`
    - ➕ **mod**
      - | FOLDERS |
        - bundles; 🔝
          - @audit : This is used in splash.rs and menu.rs ...
          but we **MUST call it in main.rs** why???
        - configs; 🔝
          - @note : lol forgot note above and fumbled around
          AGAIN with forgetting to import this here lol
      - | FILES |
        - app_state;
        - game;
        - menu;
        - splash;

  - `lib.rs`

### 📦 = ==[ Crate ]==

- 📦 **use**
  - 🍅 cargo.toml **plugins**
    - bevy::prelude::*;
    - bevy_editor_pls::prelude::*;
  - 💾 **local** crates
    - crate::app_state::AppState;
    - crate::game::GamePlugin;
    - crate::menu::MenuPlugin;
    - crate::splash::SplashPlugin;

## ⌨️ = ==[ CODE ]==

### Src/

#### | COMPILE |

##### `main.rs`

- **import**
  - | MODS |
  - | CRATES |
- `fn`
  - **main** () { ... }
    - App::new()
      - .add_plugins(**DefaultPlugins**)
      - .add_state< ... >()
        - AppState
      - .add_plugin( ... )
        - EditorPlugin::Default()
        - SplashPlugin
        - MenuPlugin
        - GamePlugin
      - .add_system_startup( ... )
        - 🎬 setup_scene
      - .add_system( ... )
        - 🎮 input_system
      - .run();
  - 🎬 **setup_scene** ( ) { } 🎬
    - ( ... )
      - mut commands: Commands
    - { ... }
      - commands.**spawn**( ... )
        - `Camera2dBundle`::default()
  - 🎮 **input_system** ( ) { } 🎮
    - ( ... )
      - **Res**< ... >
        - Input<`MouseButton`>
          - **mouse_button_click**
        - State<`AppState`>
          - **state**
      - **ResMut**< ... >
        - NextState<`AppState`>
          - mut **next_state**
    - { ... }

      - ```rust
          if mouse_button_click.just_pressed(MouseButton::Right) {
            let next = match state.0 {
                AppState::Menu => AppState::Splash,
                AppState::Splash => AppState::Menu,
                AppState::Game => AppState::Menu,
                AppState::Save => AppState::Menu,
                AppState::GameOfLife => AppState::Menu,
            };
            next_state.set(next);
          }
        ```


#### | LIBRARY |

##### `lib.rs`

#### | MODULES |

- src/
  - `app_state.rs`
  - `game.rs`
  - `menu.rs`
  - `splash.rs`
- bundles/
  - `mod.rs`
- configs/
  - `mod.rs`
