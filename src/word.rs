use a_thing::filters::contains_chars;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Rules {
    contains: String,
    not_contains: String,
    positional_contains: String,
    positional_not_contains: Vec<String>,
}

fn positional_string_to_vec(str: &str) -> Vec<Option<char>> {
    str.chars()
        .into_iter()
        .map(|char| if char == '_' { None } else { Some(char) })
        .collect()
}

pub fn get_word(rules: Rules, dict: &[String]) -> String {
    let positional_contains = positional_string_to_vec(&rules.positional_contains);
    let mut v: Vec<Vec<Option<char>>> = vec![];
    let mut filters = contains_chars(dict, rules.contains.as_str())
        .not_contains_chars(rules.not_contains.as_str())
        .positional_contains_chars(&positional_contains);

    for pc in &rules.positional_not_contains {
        v.push(positional_string_to_vec(pc));
    }
    for pc in v.iter() {
        filters = filters.positional_not_contains_chars(pc)
    }
    let words = filters.take(1).apply();
    words.get(0).unwrap_or(&"".to_string()).clone()
}
