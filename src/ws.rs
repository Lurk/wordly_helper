use crate::word::{get_word, Rules};
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Instant;

struct WordWs {
    dict: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct WsReq {
    pub id: String,
    pub rules: Rules,
}

#[derive(Serialize)]
struct WsRes {
    id: String,
    word: String,
}

impl Actor for WordWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WordWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if let Ok(req) = serde_json::from_str::<WsReq>(&text) {
                    let start = Instant::now();
                    let word = get_word(req.rules, &self.dict);
                    println!("[{}] ws word: {:?}", Utc::now(), start.elapsed());
                    ctx.text(serde_json::to_string(&WsRes { id: req.id, word }).unwrap())
                }
            }
            _ => (),
        }
    }
}

pub async fn ws_handler(
    dict: web::Data<Vec<String>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(
        WordWs {
            dict: dict.as_ref().clone(),
        },
        &req,
        stream,
    )
}
