use crate::AppState;
use bevy::prelude::*;
// use bevy_save::prelude::*;
// use std::pin::Pin;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, prelude::*};
use bevy_save::WorldSaveableExt;
pub struct GameOfLifePlugin;

#[derive(Component, Debug, Default)]
struct GameOfLife;

#[derive(Component, Default, Debug, Reflect)]
struct LastUpdate(f64);

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        // @audit : won't build if adding plugin here
        // thread 'main' panicked at 'Error adding plugin
        // bevy_save::plugins::SavePlugin: : plugin was already added in
        // application
        // ... should all plugin be added in main.rs?
        app
            // .add_plugin(SavePlugin)
            .add_plugin(TilemapPlugin)
            .add_systems((
                on_enter_game_of_life.in_schedule(OnEnter(AppState::GameOfLife)),
                on_exit_game_of_life.in_schedule(OnExit(AppState::GameOfLife)),
                AppState::on_exit_state::<GameOfLife>.in_schedule(OnExit(AppState::GameOfLife)),
            ));

        /* DEAL WITH THIS LATER LOL
        let plugins = app.get_added_plugins::<WindowPlugin>();
        Pin::new(&mut plugins[0].set(WindowPlugin{
            primary_window: Some(WindowDescriptor {
                title: "Game of Life".to_string(),
                width: 800.,
                height: 600.,
                exit_condition: Some(ExitCondition::default()),
                close_when_requested: true,
                ..Default::default()
            })
        }));
        */
    }
}

// World is the container for all ECS :
// - Resources
// - Components
// - Systems
fn on_enter_game_of_life(world: &mut World) {
    println!("on_enter_game_of_life");
    if let Err(e) = world.load("gol") {
        info!("Failed to load: {:?}", e);
        // @audit : Understand what is happening here, how are we running systems
        // off app.startup()?
        let mut system = IntoSystem::into_system(startup);
        // Initialize the system with `world` data
        system.initialize(world);
        // Runs the system with `world` data
        // - @audit : this is in unsafe mode, will come back once I understand
        system.run((), world);
        // Applies any changes that have been buffered during run to the world
        system.apply_buffers(world);
    }
    // else {
    info!("Loaded");
    // let mut system = IntoSystem::into_system(startup);
    let mut system = IntoSystem::into_system(finish_setup);
    system.initialize(world);
    system.run((), world);
    system.apply_buffers(world);
    // }
}

fn on_exit_game_of_life(world: &mut World) {
    info!("Save on Exit");
    world.save("gol").expect("Failed to save");
}

fn startup(mut commands: Commands) {
    info!("[GOL] Startup");
    let map_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let mut i = 0;
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    visible: TileVisible(i % 2 == 0 || i % 7 == 0),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
            i += 1;
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    },));
}

fn finish_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_storage_query: Query<Entity, With<TileStorage>>,
) {
    info!("[GOL] Finish Setup");
    commands.spawn((GameOfLife, Camera2dBundle::default()));

    let texture_handle: Handle<Image> = asset_server.load("icon.png");

    // @audit : IS EMPTY, TRACK HOW COME
    let is_empty = tile_storage_query.is_empty();
    println!("is tile_map empty: {}", is_empty);
    let tilemap_result = tile_storage_query.single();
    /*
    if let Ok(tilemap_entity) = tile_storage_query.single() {
        commands
            .entity(tilemap_entity)
            .insert((TilemapTexture::Single(texture_handle), LastUpdate(0.0)));
    } else {
        // You can add additional error handling here.
        println!("No Entity found or more than one Entity found");
    }
     */
    /*
       commands
           .entity(tilemap_entity)
           .insert((TilemapTexture::Single(texture_handle), LastUpdate(0.0)));
    */
}
