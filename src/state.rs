use rltk::{GameState, Rltk};
use specs::{Join, World, WorldExt};

use crate::{Position, Renderable};

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clears the console
        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(
                pos.x,
                pos.y,
                render.fg,
                render.bg,
                render.glyph,
            )
        }
    }
}