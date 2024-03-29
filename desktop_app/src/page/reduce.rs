use catan_lib::{player::TPlayer, ressource::Ressource, ressource_manager::RessourceManager};
use macroquad::prelude::*;
use macroquadstate::{
    button::Button, center::CenterH, drawable::Drawable, fix_text::FixText, h_stack::HStack,
    hstack, margin::Margin, space::Space, state::State, v_stack::VStack,
};

use crate::data::{Data, DataReturn, GamePage};

#[profiling::function]
pub fn reduce(state: &mut State<Data, DataReturn>) -> Option<VStack<4>> {
    let data = state.data();
    let max_amount = data.game.max_ressource();
    let Some((i, player)) = data
        .game
        .players()
        .iter()
        .enumerate()
        .find(|(_, player)| player.ressources().amounts() > max_amount)
    else {
        state.mutate(|data| {
            data.page = GamePage::Game;
        });
        return None;
    };
    let can_reduce = data.to_reduce.amounts() == max_amount;

    Some(VStack::new([
        Box::new(CenterH::new(Margin::news(
            FixText::new(format!("Player to reduce {}", player.name()), 25, WHITE),
            25.0,
        ))),
        Box::new(CenterH::new(hstack![
            Margin::news(
                FixText::new(
                    format!("Tree:{}", player.ressources().get(Ressource::Tree)),
                    25,
                    WHITE,
                ),
                25.0,
            ),
            Margin::news(
                FixText::new(
                    format!("Wheet:{}", player.ressources().get(Ressource::Wheet)),
                    25,
                    WHITE,
                ),
                25.0,
            ),
            Margin::news(
                FixText::new(
                    format!("Brick:{}", player.ressources().get(Ressource::Brick)),
                    25,
                    WHITE,
                ),
                25.0,
            ),
            Margin::news(
                FixText::new(
                    format!("Sheep:{}", player.ressources().get(Ressource::Sheep)),
                    25,
                    WHITE,
                ),
                25.0,
            ),
            Margin::news(
                FixText::new(
                    format!("Stone:{}", player.ressources().get(Ressource::Stone)),
                    25,
                    WHITE,
                ),
                25.0,
            ),
        ])),
        Box::new(CenterH::new(edit_row(
            max_amount,
            u8::try_from(i).expect("player outside of range"),
            state,
        ))),
        if can_reduce {
            Box::new(CenterH::new(Button::new("Reduce", state, move |data| {
                *data.game.players_mut()[i].ressources_mut() = data.to_reduce;
                data.to_reduce = RessourceManager::default();
            })))
        } else {
            Box::new(Space::new(10.0, 10.0))
        },
    ]))
}

fn edit_row(max_amount: u8, player_id: u8, state: &mut State<Data, DataReturn>) -> HStack<5> {
    HStack::new(
        [
            Ressource::Tree,
            Ressource::Wheet,
            Ressource::Brick,
            Ressource::Sheep,
            Ressource::Stone,
        ]
        .into_iter()
        .map(move |ressource| {
            let data = state.data();
            let player = data.game.player(player_id);
            let current_reduce = data.to_reduce.get(ressource);
            Box::new(Margin::news(
                VStack::new([
                    if player.ressources().get(ressource) > current_reduce
                        && data.to_reduce.amounts() < max_amount
                    {
                        Box::new(CenterH::new(Button::new("+", state, move |data| {
                            data.to_reduce.add(ressource, 1);
                        })))
                    } else {
                        Box::new(Space::new(10.0, 10.0))
                    },
                    Box::new(CenterH::new(Margin::news(
                        FixText::new(format!("{current_reduce}"), 25, WHITE),
                        10.0,
                    ))),
                    if current_reduce > 0 {
                        Box::new(CenterH::new(Button::new("-", state, move |data| {
                            data.to_reduce.sub(ressource, 1);
                        })))
                    } else {
                        Box::new(Space::new(10.0, 10.0))
                    },
                ]),
                5.0,
            )) as Box<dyn Drawable>
        })
        .collect::<Vec<Box<dyn Drawable>>>()
        .try_into()
        .ok()
        .expect("don't contait 5 element"),
    )
}
