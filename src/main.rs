use a_thing::file::from_file;
use a_thing::filters::contains_chars;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result};
use chrono::prelude::*;
use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct Rules {
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

async fn word(rules: web::Json<Rules>, dict: web::Data<Vec<String>>) -> Result<String> {
  let start = Instant::now();
  let positional_contains = positional_string_to_vec(&rules.positional_contains);
  let mut v: Vec<Vec<Option<char>>> = vec![];
  let mut filters = contains_chars(&dict, rules.contains.as_str())
    .not_contains_chars(rules.not_contains.as_str())
    .positional_contains_chars(&positional_contains);

  for pc in &rules.positional_not_contains {
    v.push(positional_string_to_vec(pc));
  }
  for pc in v.iter() {
    filters = filters.positional_not_contains_chars(pc)
  }
  let words = filters.take(1).apply();

  println!("[{}] GET word: {:?}", Utc::now(), start.elapsed());
  Ok(words.get(0).unwrap_or(&"".to_string()).clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let d = from_file("./data/sowpods_5.txt")?;
  HttpServer::new(move || {
    let cors = Cors::permissive();

    App::new()
      .wrap(cors)
      .data(d.clone())
      .route("/api/word", web::post().to(word))
  })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
