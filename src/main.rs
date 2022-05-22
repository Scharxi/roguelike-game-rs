use rltk::{RGB, RltkBuilder};
use specs::{Builder, World, WorldExt};
use crate::components::{Player, Position, Renderable, Viewshed};
use crate::map::{Map, new_map_test};
use crate::state::State;

mod state;
mod components;
mod player;
mod map;
mod math;
mod systems;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Initialize the game state
    let mut game_state = State { ecs: World::new() };

    // Register Components
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    // place the player in the center of the first room
    let (player_x, player_y) = map.rooms[0].center();

    // skip first room because the player shouldn't have a mob on top of him
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        game_state.ecs.create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('g'),
                fg: RGB::named(rltk::RED),
                ..Default::default()
            })
            .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
            .build();
    }

    game_state.ecs.insert(map);

    let player_entity = game_state.ecs.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            ..Default::default()
        })
        .with(Player{})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true
        })
        .build();

    /*
        Runs the BTerm application, calling into the provided
        game state handler every tick.
     */
    rltk::main_loop(context, game_state)
}
