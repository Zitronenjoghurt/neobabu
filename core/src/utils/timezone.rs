use chrono_tz::{Tz, TZ_VARIANTS};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn fuzzy_match_tz(search: &str, limit: usize) -> Vec<Tz> {
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<(Tz, i64)> = TZ_VARIANTS
        .iter()
        .filter_map(|tz| {
            matcher
                .fuzzy_match(tz.name(), search)
                .map(|score| (*tz, score))
        })
        .collect();

    results.sort_by_key(|(_, score)| -score);
    results.into_iter().map(|(tz, _)| tz).take(limit).collect()
}
