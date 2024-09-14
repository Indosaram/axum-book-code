use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use reqwest::Client;
use serde::Deserialize;
use axum::{body::Bytes, extract::State, http::StatusCode, routing::post, Json, Router};

type Cache = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Deserialize)]
struct Data {
    // 견종
    breed: String,
    // 사진 개수 옵션
    num_pics: Option<i32>,
}

async fn proxy_handler(State(state): State<Cache>, Json(data): Json<Data>) -> (StatusCode, Bytes) {
    // 캐시 조회
    if let Some(body) = state.lock().unwrap().get(&data.breed).cloned() {
        println!("{} 캐시 히트", &data.breed);
        return (StatusCode::OK, body);
    }

    println!("{} 캐시 미스", &data.breed);
    let mut url = format!("https://dog.ceo/api/breed/{}/images/random", &data.breed);

    if let Some(num_pics) = data.num_pics {
        url.push_str(&format!("/{}", num_pics));
    }

    // 백엔드 서버에 요청
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();

    // 응답 캐싱
    let code = res.status().as_u16();
    let body = res.bytes().await.unwrap();
    let mut cache = state.lock().unwrap();
    cache.insert(data.breed, body.clone());

    // 프록시 응답 반환
    (StatusCode::from_u16(code).unwrap(), body)
}

#[tokio::main]
async fn main() {
    let state: Cache = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/", post(proxy_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
