pub fn string_get_leading_number(value: impl AsRef<str>) -> Option<u64> {
    let end = value
        .as_ref()
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(value.as_ref().len());
    value.as_ref()[..end].parse().ok()
}
