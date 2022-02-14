mod word;
mod ws;

use crate::word::{get_word, Rules};
use crate::ws::ws_handler;
use a_thing::file::from_file;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result};
use chrono::prelude::*;
use std::time::Instant;

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
    .bind("0.0.0.0:80")?
    .run()
    .await
}
