use rltk::{console, field_of_view, Point};
use specs::prelude::*;

use crate::{Map, Monster, Name, Position, Viewshed};
use crate::math::xy_idx;

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>,);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map,player_pos, mut viewshed, monster, name, mut position) = data;

        for (viewshed,_monster, name, pos) in (&mut viewshed, &monster, &name, &mut position).join() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                console::log(&format!("{} shouts insults", name.name));
                //wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
            }
            else if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} shouts insults", name.name));
                let path = rltk::a_star_search(
                    xy_idx(pos.x, pos.y) as i32,
                    xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}

