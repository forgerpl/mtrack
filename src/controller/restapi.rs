// Copyright (C) 2024 Michael Wilson <mike@mdwn.dev>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//
use axum::{
    self,
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
};
use std::{io, sync::Arc};

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::{
    sync::{mpsc::Sender, watch},
    task::JoinHandle,
};
use tracing::{info, span, Level};

use super::{Event, StatusEvent};

mod ui;

/// A controller that controls a player using the keyboard.
pub struct Driver {}

impl Driver {
    pub fn new() -> Driver {
        Driver {}
    }
}

impl super::Driver for Driver {
    fn monitor_events(
        &self,
        events_tx: Sender<Event>,
        status_rx: watch::Receiver<StatusEvent>,
    ) -> JoinHandle<Result<(), io::Error>> {
        tokio::task::spawn(async move {
            info!("starting restapi driver");

            let span = span!(Level::INFO, "restapi driver");
            let _enter = span.enter();

            struct AppState {
                events_tx: Sender<Event>,
                status_rx: watch::Receiver<StatusEvent>,
            }

            let state = Arc::new(AppState {
                events_tx,
                status_rx,
            });

            let api = axum::Router::new()
                .route(
                    "/play",
                    axum::routing::post({
                        let state = Arc::clone(&state);
                        || async move {
                            info!("received play api request");
                            state.events_tx.send(Event::Play).await.unwrap();
                        }
                    }),
                )
                .route(
                    "/prev",
                    axum::routing::post({
                        let state = Arc::clone(&state);
                        || async move {
                            info!("received prev api request");
                            state.events_tx.send(Event::Prev).await.unwrap();
                        }
                    }),
                )
                .route(
                    "/next",
                    axum::routing::post({
                        let state = Arc::clone(&state);
                        || async move {
                            info!("received next api request");
                            state.events_tx.send(Event::Next).await.unwrap();
                        }
                    }),
                )
                .route(
                    "/stop",
                    axum::routing::post({
                        let state = Arc::clone(&state);
                        || async move {
                            info!("received stop api request");
                            state.events_tx.send(Event::Stop).await.unwrap();
                        }
                    }),
                )
                .route(
                    "/allsongs",
                    axum::routing::post({
                        let state = Arc::clone(&state);
                        || async move {
                            info!("received allsongs api request");
                            state.events_tx.send(Event::AllSongs).await.unwrap();
                        }
                    }),
                )
                .route(
                    "/playlist",
                    axum::routing::post({
                        let state = Arc::clone(&state);
                        || async move {
                            info!("received playlist api request");
                            state.events_tx.send(Event::Playlist).await.unwrap();
                        }
                    }),
                )
                .route(
                    "/state",
                    axum::routing::get({
                        let status_rx = state.status_rx.clone();

                        || async move {
                            info!("received state api request");
                            axum::Json(status_rx.borrow().clone())
                        }
                    }),
                );

            let wsapi = axum::Router::new()
                .route(
                    "/ws",
                    axum::routing::get({
                        let state = Arc::clone(&state);
                        move |ws: WebSocketUpgrade| async move {
                            info!("upgrading websocket connection");

                            ws.on_upgrade(move |socket: WebSocket| async move {
                                info!("upgrading websocket connection");
                                let mut status_rx = state.status_rx.clone();

                                let (mut sender, mut receiver) = socket.split();

                                loop {
                                    tokio::select! {
                                        Ok(_) = status_rx.changed() => {
                                            let payload = serde_json::to_string(&*status_rx.borrow()).unwrap();
                                            let _ = sender.send(Message::Text(payload)).await;
                                        },
                                        Some(Ok(message)) = receiver.next() => {
                                            match message {
                                                Message::Ping(payload) => {
                                                    info!("Got ping");
                                                    let _ = sender.send(Message::Pong(payload)).await;
                                                }
                                                Message::Pong(_payload) => {
                                                    info!("Got pong");
                                                }
                                                Message::Close(_) => {
                                                    info!("Got close message");
                                                    break;
                                                }
                                                Message::Binary(_) => {
                                                    info!("Got binary message");
                                                }
                                                Message::Text(data) => {
                                                    info!("Got message {data:?}");
                                                }
                                            }
                                        }
                                    }
                                }

                                sender.reunite(receiver).unwrap().close().await.unwrap();
                            })
                        }
                    }),
                )
                .with_state(state);

            let app = axum::Router::new()
                .nest("/", ui::router())
                .nest("/", wsapi)
                .nest("/api", api);

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await?;
            axum::serve(listener, app).await?;

            Ok(())
        })
    }
}
