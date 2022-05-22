use rltk::{Point, RGB, RltkBuilder};
use specs::{Builder, World, WorldExt};

use crate::components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed};
use crate::map::{Map, new_map_test};
use crate::state::{RunState, State};

mod state;
mod components;
mod player;
mod map;
mod math;
mod systems;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // Initialize the game state
    let mut game_state = State { ecs: World::new(), run_state: RunState::Running };

    // Register Components
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Monster>();
    game_state.ecs.register::<Name>();
    game_state.ecs.register::<BlocksTile>();
    game_state.ecs.register::<CombatStats>();

    let map = Map::new_map_rooms_and_corridors();
    // place the player in the center of the first room
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    // skip first room because the player shouldn't have a mob on top of him
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        let name: String;

        match roll {
            // goblin
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_owned()
            }
            // orc
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_owned();
            }
        }

        game_state.ecs.create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                ..Default::default()
            })
            .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
            .with(Monster {})
            .with(Name { name: format!("{} #{}", &name, i) })
            .with(BlocksTile{})
            .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
            .build();
    }

    game_state.ecs.insert(map);

    let player_entity = game_state.ecs.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            ..Default::default()
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {name: "Player".to_owned()})
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .build();

    game_state.ecs.insert(Point::new(player_x, player_y));

    /*
        Runs the BTerm application, calling into the provided
        game state handler every tick.
     */
    rltk::main_loop(context, game_state)
}
