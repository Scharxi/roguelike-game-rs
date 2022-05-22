use rltk::RltkBuilder;
use crate::state::State;

mod state;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Initialize the game state
    let game_state = State {};

    /*
        Runs the BTerm application, calling into the provided
        game state handler every tick.
     */
    rltk::main_loop(context, game_state)
}
