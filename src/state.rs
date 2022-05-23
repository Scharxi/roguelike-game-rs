use rltk::{GameState, Rltk};
use specs::{Join, RunNow, World, WorldExt};

use crate::{Map, Position, Renderable, systems};
use crate::map::{draw_map, TileType};
use crate::math::xy_idx;
use crate::player::player_input;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput, PreRun, PlayerTurn, MonsterTurn
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = systems::VisibilitySystem;
        vis.run_now(&self.ecs);
        let mut ai = systems::MonsterAI;
        ai.run_now(&self.ecs);
        let mut indexing = systems::MapIndexingSystem;
        indexing.run_now(&self.ecs);
        let mut melee = systems::MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut damage = systems::DamageSystem{};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clears the console
        ctx.cls();
        let mut new_run_state;

        {
            let run_state = self.ecs.fetch::<RunState>();
            new_run_state = *run_state;
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_run_state = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_run_state;
        }

        systems::delete_the_dead(&mut self.ecs);

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = xy_idx(pos.x, pos.y);

            if map.visible_tiles[idx] {
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
}