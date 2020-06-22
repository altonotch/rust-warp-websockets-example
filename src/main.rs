#[macro_use]
extern crate log;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use pretty_env_logger;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use structopt::StructOpt;
use tokio::time::Duration;
use warp::filters::ws::{Message, WebSocket};
use warp::Filter;

#[derive(StructOpt, Debug)]
#[structopt(name = "websocket-server")]
struct Opts {
    /// Optional port to run on.
    #[structopt(short, long, default_value = "7878")]
    port: u16,
}

#[derive(Deserialize, Debug)]
struct WsRequest {
    kind: String,
    message: String,
}

#[derive(Serialize)]
struct WsResult {
    status: String,
    response: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let opt = Opts::from_args();

    info!("initializing server on port: {}", opt.port);

    let health_check = warp::path("health-check").map(|| format!("Server OK"));
    let ws = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        info!("upgrading connection to websocket");
        ws.on_upgrade(handle_ws_client)
    });

    let routes = health_check.or(ws).with(warp::cors().allow_any_origin());

    warp::serve(routes)
        .run(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            opt.port,
        ))
        .await;
    info!("server is running");
}

async fn handle_ws_client(websocket: warp::ws::WebSocket) {
    // receiver - this server, from websocket client
    // sender - diff clients connected to this server
    let (mut sender, mut receiver) = websocket.split();

    while let Some(body) = receiver.next().await {
        let message = match body {
            Ok(msg) => msg,
            Err(e) => {
                error!("error reading message on websocket: {}", e);
                break;
            }
        };

        handle_websocket_message(message, &mut sender).await;
    }

    info!("client disconnected");
}

async fn handle_websocket_message(message: Message, sender: &mut SplitSink<WebSocket, Message>) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = message.to_str() {
        s
    } else {
        info!("ping-ping");
        return;
    };

    let req: WsRequest = serde_json::from_str(msg).unwrap();
    info!("got request {} with body {}", req.kind, req.message);

    std::thread::sleep(Duration::new(1, 0));

    let response = serde_json::to_string(&WsResult {
        status: "success".to_string(),
        response: "awesome message".to_string(),
    })
    .unwrap();
    sender.send(Message::text(response)).await.unwrap();
}
