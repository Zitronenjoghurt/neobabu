use crate::error::BotError;
use crate::state::BotState;
use poise::Command;

mod birthday;
mod ping;

pub fn get_commands() -> Vec<Command<BotState, BotError>> {
    vec![ping::ping(), birthday::birthday()]
}
