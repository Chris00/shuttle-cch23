use std::{collections::HashMap, sync::Arc};
use axum::{
    extract::{Path, State, ws::{WebSocketUpgrade, WebSocket, Message}},
    response::Response,
};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{self, Sender};

use crate::AppState;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Room(i32);

#[derive(Debug)]
pub struct Views {
    total: u32,
    rooms: HashMap<Room, Arc<Sender<BroadcastTweet>>>,
}

impl Views {
    pub fn new() -> Self {
        Views { total: 0,  rooms: HashMap::new() }
    }
}

pub async fn reset(State(state): State<super::AppState>) {
    state.day19.lock().unwrap().total = 0;
}

pub async fn views(State(state): State<super::AppState>) -> String {
    let v = &mut state.day19.lock().unwrap();
    format!("{}", v.total)
}

pub async fn room(
    State(state): State<super::AppState>,
    Path((room, user)): Path<(i32, String)>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| {
        room_ws(socket, state, Room(room), user)
    })
}

#[derive(Deserialize, Debug)]
struct Tweet {
    message: String,

}

#[derive(Serialize, Debug, Clone)]
struct BroadcastTweet {
    user: String,
    message: String,
}

async fn room_ws(
    mut socket: WebSocket, state: AppState,
    room: Room, user: String,
) {
    let tx = state.day19.lock().unwrap().rooms.entry(room)
        .or_insert(Arc::new(broadcast::channel(20000).0))
        .clone();
    let mut rx = tx.subscribe();
    loop {
        tokio::select!(
            msg = socket.recv() => {
                let msg = if let Some(Ok(msg)) = msg { msg } else { return };
                let msg = match msg {
                    Message::Text(t) => t,
                    Message::Close(_) => return,
                    _ => continue,
                };
                if let Ok(tweet) = serde_json::from_str::<Tweet>(&msg) {
                    if tweet.message.len() > 128 { continue }

                    tx.send(BroadcastTweet {
                        user: user.clone(),
                        message: tweet.message
                    }).unwrap();
                }
            }
            Ok(tweet) = rx.recv() => {
                let msg = serde_json::to_string(&tweet).unwrap();
                if socket.send(Message::Text(msg)).await.is_ok() {
                    // Successful sent
                    state.day19.lock().unwrap().total += 1;
                } else {
                    return; // Client disconnected
                }
            }
        )
    }
}

pub async fn ping(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ping_game)
}

async fn ping_game(mut socket: WebSocket) {
    let mut game_started = false;
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return; // client disconnected
        };

        if let Message::Text(txt) = msg {
            if &txt == "serve" {
                game_started = true
            }
            if game_started {
                if &txt == "ping" {
                    let msg = Message::Text("pong".to_string());
                    if socket.send(msg).await.is_err() {
                        return; // Client disconnected
                    }
                }
            }
        }
    }
}
