use std::cmp::{max, min};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::{Join, WorldExt};
use crate::{Map, Player, Position, RunState, State, Viewshed, World};
use crate::map::{TileType};
use crate::math::xy_idx;

pub fn try_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewshed).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        // check if current tile is a wall
        if map.tiles[destination_idx] != TileType::Wall {
            // moves player if its not a wall
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::Paused },
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => try_move(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => try_move(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => try_move(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => try_move(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
    RunState::Running
}

