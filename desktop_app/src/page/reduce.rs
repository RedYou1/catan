use catan_lib::{
    game_manager::Game, player::TPlayer, ressource::Ressource, ressource_manager::RessourceManager,
};
use macroquad::{prelude::*, ui::root_ui};

use crate::{
    draw::{texts_horizontal::texts_horizontal, texts_vertical::texts_vertical},
    player::Player,
    state::State,
    Page,
};

pub fn reduce(game: &mut Game<Player, 4>, state: &mut State) {
    let max_amount = game.max_ressource();
    let Some(player) = game
        .players_mut()
        .iter_mut()
        .find(|player| player.ressources().amounts() > max_amount)
    else {
        state.page = Page::Main;
        return;
    };

    let next_y = texts_vertical(
        &[format!("Player to reduce {}", player.name())],
        screen_width() / 2.0,
        100.0,
        5.0,
        25,
        WHITE,
    );

    let ressources = player.ressources_mut();
    texts_horizontal(
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
        let current_reduce = state.to_reduce.get(ressource);
        if ressources.get(ressource) > current_reduce
            && state.to_reduce.amounts() < max_amount
            && root_ui().button(
                Vec2 {
                    x: i * 100.0,
                    y: next_y + 100.0,
                },
                "+",
            )
        {
            state.to_reduce.add(ressource, 1);
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
            state.to_reduce.sub(ressource, 1);
        }
    }

    if state.to_reduce.amounts() == max_amount
        && root_ui().button(
            Vec2 {
                x: screen_width() / 2.0 - 25.0,
                y: next_y + 225.0,
            },
            "Reduce",
        )
    {
        *ressources = state.to_reduce;
        state.to_reduce = RessourceManager::default();
    }
}
