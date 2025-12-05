use crate::Context;
use neobabu_core::utils::timezone::fuzzy_match_tz;

pub async fn autocomplete_timezone<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    let candidates = fuzzy_match_tz(partial, 25);
    candidates.into_iter().map(|tz| tz.name().to_string())
}
