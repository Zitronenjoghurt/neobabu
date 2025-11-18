use crate::error::BotError;
use crate::state::BotState;
use poise::Command;

mod birthday;
mod game;
mod ping;

pub fn get_commands() -> Vec<Command<BotState, BotError>> {
    vec![ping::ping(), game::game(), birthday::birthday()]
}
