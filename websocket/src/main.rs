use std::sync::Arc;

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::{HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};

#[derive(Debug, Clone)]
struct AppState {
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let app = AppState {
        broadcast_tx: Arc::new(Mutex::new(tx)),
    };

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route_layer(middleware::from_fn(authenticate))
        .with_state(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn authenticate(
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if headers
        .get("Authorization")
        .map(|value| value == "Bearer token")
        .unwrap_or(false)
    {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn websocket_handler(ws: WebSocketUpgrade, State(app): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, app))
}

async fn handle_socket(ws: WebSocket, app: AppState) {
    let (ws_tx, ws_rx) = ws.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    {
        let broadcast_rx = app.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            recv_broadcast(ws_tx, broadcast_rx).await;
        });
    }

    recv_from_client(ws_rx, app.broadcast_tx).await;
}

async fn recv_from_client(
    mut client_rx: SplitStream<WebSocket>,
    broadcast_tx: Arc<Mutex<Sender<Message>>>,
) {
    while let Some(Ok(msg)) = client_rx.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }
        if broadcast_tx.lock().await.send(msg).is_err() {
            println!("Failed to broadcast a message");
        }
    }
}

async fn recv_broadcast(
    client_tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    mut broadcast_rx: Receiver<Message>,
) {
    while let Ok(msg) = broadcast_rx.recv().await {
        if client_tx.lock().await.send(msg).await.is_err() {
            return; // 오류 발생 시 연결 해제
        }
    }
}
