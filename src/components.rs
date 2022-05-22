use rltk::RGB;
use specs_derive::Component;
use specs::prelude::*;


#[derive(Component, Debug)]
pub struct Player;


#[derive(Component)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32
}

#[derive(Component)]
pub struct Renderable {
    pub(crate) glyph: rltk::FontCharType,
    pub(crate) fg: RGB,
    pub(crate) bg: RGB
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            glyph: rltk::to_cp437('u'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK)
        }
    }
}