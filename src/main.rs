#![feature(const_trait_impl, effects)]

use crate::game_manager::Game;
use crate::ressource::Ressource;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use ressource_manager::RessourceManager;
use tile::Tile;

mod building;
mod game_manager;
mod player;
mod port;
mod position;
mod ressource;
mod ressource_manager;
mod tile;

const HEX_SIZE: f32 = 50.0;

fn configure_window() -> Conf {
    Conf {
        window_title: String::from("Catan"),
        window_resizable: true,
        ..Default::default()
    }
}

fn draw_texts_vertical(
    texts: &[String],
    centerx: f32,
    mut starty: f32,
    font: u16,
    color: Color,
) -> f32 {
    for text in texts {
        let center = get_text_center(text, None, font, 1.0, 0.0);

        starty -= center.y * 2.0;

        draw_text(text, centerx - center.x, starty, f32::from(font), color);
    }
    starty
}

fn draw_texts_horizontal(
    texts: &[String],
    mut startx: f32,
    y: f32,
    space: f32,
    font: u16,
    color: Color,
) -> f32 {
    for text in texts {
        let center = get_text_center(text, None, font, 1.0, 0.0);

        draw_text(text, startx, y, f32::from(font), color);

        startx += center.x * 2.0 + space;
    }
    startx
}

fn draw_tile(x: usize, y: usize, starty: f32, tile: Option<Tile>) {
    let color = if let Some(tile) = tile {
        match tile.ressource() {
            Ressource::Tree => GREEN,
            Ressource::Wheet => GOLD,
            Ressource::Brick => RED,
            Ressource::Sheep => LIGHTGRAY,
            Ressource::Stone => GRAY,
        }
    } else {
        YELLOW
    };

    let px = f32::from(i16::try_from(x).expect("number try_from") - 3);
    let py = f32::from(i16::try_from(y).expect("number try_from"));
    let isoff = f32::from(u8::try_from(y % 2).expect("number try_from")) / 2.0;

    draw_hexagon(
        screen_width() / 2.0 + 1.8 * HEX_SIZE * (px + isoff),
        starty + HEX_SIZE * 2.0 + 1.54 * HEX_SIZE * py,
        HEX_SIZE,
        0.0,
        true,
        BLANK,
        color,
    );

    if let Some(tile) = tile {
        draw_texts_vertical(
            &[
                tile.dice_id().to_string(),
                str::repeat(
                    "*",
                    usize::try_from((7 - i32::from(tile.dice_id())).abs())
                        .expect("number try_from"),
                ),
            ],
            screen_width() / 2.0 + 1.8 * HEX_SIZE * (px + isoff),
            starty + HEX_SIZE * 2.0 + 1.54 * HEX_SIZE * py - HEX_SIZE / 4.0,
            25,
            BLACK,
        );
    }
}

enum Page {
    Main,
    Reduce,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_lines)]
#[macroquad::main(configure_window)]
async fn main() {
    #[deny(clippy::needless_pass_by_value)]
    let mut game =
        Game::new(7, ["Blue", "Red", "Green", "Yellow"]).expect("Couldn't create the game");
    let mut page = Page::Main;
    let mut to_reduce = RessourceManager::default();
    let mut dices: Option<(u8, u8)> = None;
    loop {
        clear_background(DARKGRAY);

        match page {
            Page::Main => main_page(&mut game, &mut page, &mut to_reduce, &mut dices),
            Page::Reduce => reduce_page(&mut game, &mut page, &mut to_reduce),
        }

        next_frame().await;
    }
}

fn main_page(
    game: &mut Game<4>,
    page: &mut Page,
    to_reduce: &mut RessourceManager,
    dice_played: &mut Option<(u8, u8)>,
) {
    let mut new_y = draw_texts_vertical(
        &[format!("Player to play: {}", game.current_player().name())],
        screen_width() / 2.0,
        0.0,
        25,
        WHITE,
    );
    new_y = draw_texts_vertical(
        &game
            .players()
            .iter()
            .map(|player| {
                format!(
                    "Ressource of Player: {}, {:?}",
                    player.name(),
                    player.ressources()
                )
            })
            .collect::<Vec<String>>(),
        screen_width() / 2.0,
        new_y,
        20,
        WHITE,
    );

    for (y, row) in game.tiles().iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if (x == 0 || x == 4) && (y == 0 || y == 4) {
                continue;
            }
            if x == 4 && (y == 1 || y == 3) {
                continue;
            }
            draw_tile(x, y, new_y, *tile);
        }
    }

    if let Some((a, b)) = *dice_played {
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
            *dice_played = None;
        }
    } else if root_ui().button(
        Vec2 {
            x: screen_width() / 2.0,
            y: screen_height() - 40.0,
        },
        "Dice",
    ) {
        let (a, b) = game.throw_dice();
        *dice_played = Some((a, b));
        if a + b == 7 {
            *to_reduce = RessourceManager::default();
            *page = Page::Reduce;
        }
    }
}

fn reduce_page(game: &mut Game<4>, page: &mut Page, to_reduce: &mut RessourceManager) {
    let max_amount = game.max_ressource();
    let Some(player) = game
        .players_mut()
        .iter_mut()
        .find(|player| player.ressources().amounts() > max_amount)
    else {
        *page = Page::Main;
        return;
    };

    let next_y = draw_texts_vertical(
        &[format!("Player to reduce {}", player.name())],
        screen_width() / 2.0,
        100.0,
        25,
        WHITE,
    );

    let ressources = player.ressources_mut();
    draw_texts_horizontal(
        &[
            format!("Tree:{}", ressources.get(Ressource::Tree)),
            format!("Wheet:{}", ressources.get(Ressource::Wheet)),
            format!("Brick:{}", ressources.get(Ressource::Brick)),
            format!("Sheep:{}", ressources.get(Ressource::Sheep)),
            format!("Stone:{}", ressources.get(Ressource::Stone)),
        ],
        100.0,
        next_y + 25.0,
        25.0,
        25,
        WHITE,
    );

    for (i, ressource) in [
        (1.0, Ressource::Tree),
        (2.0, Ressource::Wheet),
        (3.0, Ressource::Brick),
        (4.0, Ressource::Sheep),
        (5.0, Ressource::Stone),
    ] {
        let current_reduce = to_reduce.get(ressource);
        if ressources.get(ressource) > current_reduce
            && ressources.amounts() - to_reduce.amounts() > max_amount
            && root_ui().button(
                Vec2 {
                    x: i * 100.0,
                    y: next_y + 100.0,
                },
                "+",
            )
        {
            to_reduce.add(ressource, 1);
        }

        draw_text(
            format!("{current_reduce}").as_str(),
            i * 100.0,
            next_y + 150.0,
            25.0,
            WHITE,
        );

        if current_reduce > 0
            && root_ui().button(
                Vec2 {
                    x: i * 100.0,
                    y: next_y + 175.0,
                },
                "-",
            )
        {
            to_reduce.sub(ressource, 1);
        }
    }

    if ressources.amounts() - to_reduce.amounts() == max_amount
        && root_ui().button(
            Vec2 {
                x: screen_width() / 2.0 - 25.0,
                y: next_y + 225.0,
            },
            "Reduce",
        )
    {
        ressources.subs(*to_reduce);
        *to_reduce = RessourceManager::default();
    }
}
