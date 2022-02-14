mod word;
mod ws;

use crate::word::{get_word, Rules};
use crate::ws::ws_handler;
use a_thing::file::from_file;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result, Responder};
use chrono::prelude::*;
use std::time::Instant;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Time {
  step: String,
  time: i64,
}

#[derive(Serialize, Deserialize)]
struct Req {
  rules: Rules,
  times: Vec<Time>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Res {
  word: String,
  times: Vec<Time>,
}

async fn word(req: web::Json<Req>, dict: web::Data<Vec<String>>) -> Result<impl Responder> {
  let mut times = req.times.clone();
  times.push(Time { step: "get_request".to_string(), time: Utc::now().timestamp_millis() });
  let start = Instant::now();
  let word = get_word(&req.rules, &dict);
  println!("[{}] GET word: {:?}", Utc::now(), start.elapsed());
  times.push(Time { step: "found a word".to_string(), time: Utc::now().timestamp_millis() });
  Ok(web::Json(Res { word, times }))
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
      .route("/api/ws", web::get().to(ws_handler))
  })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
