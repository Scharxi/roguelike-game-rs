use rltk::{RGB, RltkBuilder};
use specs::{Builder, World, WorldExt};
use crate::components::{Player, Position, Renderable};
use crate::map::{Map, new_map_test};
use crate::state::State;

mod state;
mod components;
mod player;
mod map;
mod math;

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

    let map = Map::new_map_rooms_and_corridors();
    // place the player in the center of the first room
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = game_state.ecs.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            ..Default::default()
        })
        .with(Player{})
        .build();

    game_state.ecs.insert(map);


    /*
        Runs the BTerm application, calling into the provided
        game state handler every tick.
     */
    rltk::main_loop(context, game_state)
}
