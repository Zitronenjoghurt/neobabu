use crate::context::ContextExt;
use crate::Context;
use neobabu_core::utils::string::string_get_leading_number;

pub async fn autocomplete_world_name<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = String> + 'a {
    let mut worlds = ctx
        .services()
        .farming
        .fuzzy_search_worlds(ctx.author_id_string(), partial, 25)
        .await
        .unwrap_or_default();
    worlds.sort_by_key(|world| world.index);

    worlds
        .into_iter()
        .map(|world| format!("{} - {}", world.index + 1, world.name))
}

pub fn autocomplete_extract_world_index(value: &str) -> usize {
    string_get_leading_number(value)
        .unwrap_or_default()
        .saturating_sub(1) as usize
}
