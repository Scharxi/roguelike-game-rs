use std::cmp::{max, min};
use rltk::{Rltk, VirtualKeyCode};
use specs::{Join, WorldExt};
use crate::{Player, Position, State, World};

pub fn try_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => try_move(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => try_move(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => try_move(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => try_move(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

