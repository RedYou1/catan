use catan_lib::{game_manager::Game, player::TPlayer, ressource_manager::RessourceManager};
use macroquad::{prelude::*, ui::root_ui};

use crate::{
    draw::{
        building::building,
        road::{hroad, vroad},
        texts_vertical::texts_vertical,
        tile,
    },
    player::Player,
    state::{State, Thief},
    Page,
};

pub fn game(game: &mut Game<Player, 4>, state: &mut State) {
    let current_player = game.current_player();
    let mut new_y = texts_vertical(
        &[format!("Player to play: {}", current_player.name())],
        screen_width() / 2.0,
        0.0,
        5.0,
        25,
        WHITE,
    );
    new_y = texts_vertical(
        &game
            .players()
            .iter()
            .map(|player| {
                format!(
                    "Player:{} LR:{} Army:{} {}",
                    player.name(),
                    player.longuest_road(),
                    player.army(),
                    player.ressources().to_string()
                )
            })
            .collect::<Vec<String>>(),
        screen_width() / 2.0,
        new_y,
        5.0,
        20,
        WHITE,
    );
    new_y -= 30.0;

    draw_tiles(new_y, game, state);
    draw_roads(new_y, game, state);
    draw_buildings(new_y, game, state);

    if state.debut.is_starting() {
        return;
    }
    if let Some((a, b)) = state.dices {
        if state.thief == Thief::Waiting {
            return;
        }

        draw_text(
            format!("{a} {b}").as_str(),
            screen_width() / 2.0,
            screen_height() - 40.0,
            25.0,
            WHITE,
        );
        if root_ui().button(
            Vec2 {
                x: screen_width() / 2.0,
                y: screen_height() - 20.0,
            },
            "Next",
        ) {
            game.next_player();
            state.dices = None;
        }
    } else if root_ui().button(
        Vec2 {
            x: screen_width() / 2.0,
            y: screen_height() - 40.0,
        },
        "Dice",
    ) {
        let (a, b) = game.throw_dice();
        state.dices = Some((a, b));
        state.thief = if a + b == 7 {
            Thief::Waiting
        } else {
            Thief::No
        };
        if a + b == 7 {
            state.to_reduce = RessourceManager::default();
            state.page = Page::Reduce;
        }
    }
}

pub fn draw_tiles(new_y: f32, game: &mut Game<Player, 4>, state: &mut State) {
    for y in 0..5 {
        for x in 0..5 {
            if (x == 0 || x == 4) && (y == 0 || y == 4) {
                continue;
            }
            if x == 4 && (y == 1 || y == 3) {
                continue;
            }
            tile::tile(x, y, new_y, game, state);
        }
    }
}

pub fn draw_buildings(new_y: f32, game: &mut Game<Player, 4>, state: &mut State) {
    for y in 0..6 {
        for x in 0..11 {
            if (y == 0 || y == 5) && (x <= 1 || x >= 9) {
                continue;
            }
            if (y == 1 || y == 4) && (x == 0 || x == 10) {
                continue;
            }
            building(x, y, new_y, game, state);
        }
    }
}

pub fn draw_roads(new_y: f32, game: &mut Game<Player, 4>, state: &mut State) {
    for y in 0..5 {
        for x in 0..6 {
            if (y == 0 || y == 4) && (x == 0 || x == 5) {
                continue;
            }
            if (y == 1 || y == 3) && x == 5 {
                continue;
            }
            vroad(x, y, new_y, game, state);
        }
    }

    for y in 0..6 {
        for x in 0..10 {
            if (y == 0 || y == 5) && (x <= 1 || x >= 8) {
                continue;
            }
            if (y == 1 || y == 4) && (x == 0 || x == 9) {
                continue;
            }
            hroad(x, y, new_y, game, state);
        }
    }
}
