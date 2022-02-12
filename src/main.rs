mod word;

use crate::word::{get_word, Rules};
use a_thing::file::from_file;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result, HttpRequest, HttpResponse, Error};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use chrono::prelude::*;
use std::time::Instant;

struct WordWs {
  dict: Vec<String>
}

impl Actor for WordWs {
  type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WordWs {
  fn handle(
    &mut self,
    msg: Result<ws::Message, ws::ProtocolError>,
    ctx: &mut Self::Context,
  ) {
    match msg {
      Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
      Ok(ws::Message::Text(text)) => {
        if let Ok(rules) = serde_json::from_str(&text) {
          let start = Instant::now();
          let word = get_word(rules, &self.dict);
          println!("[{}] ws word: {:?}", Utc::now(), start.elapsed());
          ctx.text(word)
        }
      },
      _ => (),
    }
  }
}

async fn ws_handler(dict: web::Data<Vec<String>>, req: HttpRequest, stream: web::Payload)->Result<HttpResponse, Error> {
  ws::start(WordWs {dict:dict.as_ref().clone()}, &req, stream)
}

async fn word(rules: web::Json<Rules>, dict: web::Data<Vec<String>>) -> Result<String> {
    let start = Instant::now();
    let word = get_word(rules.into_inner(), &dict);
    println!("[{}] GET word: {:?}", Utc::now(), start.elapsed());
    Ok(word)
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
