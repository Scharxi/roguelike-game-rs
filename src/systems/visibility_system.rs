use rltk::{field_of_view, Point};
use specs::prelude::*;
use crate::{Map, Position, Viewshed};

pub struct VisibilitySystem;

impl <'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       ReadExpect<'a, Map>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut viewshed, pos, map) = data;

        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height );
        }
    }
}