use rltk::{GameState, Rltk};
use specs::{Join, World, WorldExt};

use crate::{Map, Position, Renderable};
use crate::map::{draw_map, TileType};
use crate::player::player_input;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clears the console
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        draw_map(&map.tiles, ctx);

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