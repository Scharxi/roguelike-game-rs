use rltk::{Rltk, GameState};

pub struct State;

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clears the console
        ctx.cls();

        // Prints "Hello Rust World" on the console
        ctx.print(1, 1, "Hello Rust World");
    }
}