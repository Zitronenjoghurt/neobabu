use crate::error::BotError;
use crate::state::BotState;
use poise::Command;

mod birthday;
mod farm;
mod game;
mod owner;
mod ping;
mod settings;
mod space;
mod youtube;

pub fn get_commands() -> Vec<Command<BotState, BotError>> {
    vec![
        ping::ping(),
        game::game(),
        birthday::birthday(),
        farm::farm(),
        owner::sync_guild(),
        settings::settings(),
        space::space(),
        youtube::youtube(),
    ]
}
