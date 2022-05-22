use rltk::{RGB, RltkBuilder};
use specs::{Builder, World, WorldExt};
use crate::components::{Player, Position, Renderable};
use crate::state::State;

mod state;
mod components;
mod player;

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

    let player_entity = game_state.ecs.create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            ..Default::default()
        })
        .with(Player{})
        .build();

    let enemy_entity = game_state.ecs.create_entity()
        .with(Position { x: 20, y: 40 })
        .with(Renderable {
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::RED),
            ..Default::default()
        })
        .build();


    /*
        Runs the BTerm application, calling into the provided
        game state handler every tick.
     */
    rltk::main_loop(context, game_state)
}
