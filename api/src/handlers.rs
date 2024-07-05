use std::collections::HashMap;
use axum::extract::{Request, State};
use axum::http::{StatusCode};
use axum::Json;
use axum::response::Response;
use sea_orm::DbErr;
use sha1::Digest;
use service::subscription::subscription::Subscription;
use service::user::user::User;
use crate::setup::{AppState, Context};

pub(crate) async fn subscribe(
    ctx: State<Context>,
    Json(data): Json<crate::req_models::Subscribe>,
) ->StatusCode {
    let res = ctx.subscription_service.subscribe(data.email.as_str(), data.account_ids)
        .await;
    match res {
        Ok(_) =>  StatusCode::OK,
        Err(_) =>  StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub(crate) async fn test_wechat(
    mut req: Request
) -> StatusCode {
    let queries = req
        .uri()
        .query()
        .unwrap_or("");

    if queries.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    let mut map: HashMap<&str, &str> = HashMap::new();

    queries.split('&').for_each(|query| {
        let mut pair = query.split('=');
        let key = pair.next().unwrap();
        let value = pair.next().unwrap();
        println!("{}: {}", key, value);
        map.insert(key, value);
    });

    let signature = *map.get("signature").unwrap();
    let timestamp = *map.get("timestamp").unwrap();
    let nonce = *map.get("nonce").unwrap();
    let TOKEN = "ADASDSAD";

    let mut vec: Vec<&str> = vec![TOKEN, timestamp, nonce];
    vec.sort();

    let mut str = String::new();
    vec.iter().for_each(|&s| {
        str.push_str(s);
    });

    let mut hasher = sha1::Sha1::new();
    hasher.update(str.as_bytes());

    let result = hasher.finalize();
    let result = hex::encode(result);
    println!("{:?}\n{:?}", result, signature);
    if result.as_str() != signature {
        return StatusCode::INTERNAL_SERVER_ERROR
    }

    return StatusCode::OK

}