use axum::extract::{Path, State};
use axum::http::{StatusCode};
use axum::Json;
use sea_orm::DbErr;
use service::subscription::subscription::{SubscriptionTrait};
use service::user::user::UserTrait;
use crate::req_models::{CancelSubscriptions, SignIn};
use crate::setup::{Context};

pub(crate) async fn subscribe(
    ctx: State<Context>,
    Json(mut data): Json<crate::req_models::Subscribe>,
) -> Result<Json<Vec<i64>>, StatusCode> {
    let res = ctx.user_service.first_or_create(data.email.as_str())
        .await;
    let user;
    let mut player_account_ids;
    match res {
        Ok(t) => {
            user = t.0;
            player_account_ids = t.1
        }
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let res = ctx.subscription_service.subscribe(user.id, data.account_ids.clone())
        .await;
    match res {
        Ok(_) => {
            player_account_ids.append(&mut data.account_ids);
            Ok(Json(player_account_ids))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(crate) async fn cancel_subscriptions(
    ctx: State<Context>,
    Json(data): Json<CancelSubscriptions>,
) -> Result<Json<Vec<i64>>, StatusCode> {
    let res = ctx.user_service.first_or_create(data.email.as_str())
        .await;
    let user;
    let player_account_ids;
    match res {
        Ok(t) => {
            user = t.0;
            player_account_ids = t.1
        }
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let res = ctx.subscription_service.cancel_subscriptions(user.id, data.account_ids.clone())
        .await;
    match res {
        Ok(_) => {
            let res = ctx.subscription_service.get_subscriptions(user.id)
                .await;
            return match res {
                Ok(ids) => Ok(Json(ids)),
                Err(_) => Ok(Json(player_account_ids)), // use the old list
            };
        }
        Err(_) => {
            eprintln!("error handling cancel subscription");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn sign_in(
    ctx: State<Context>,
    Json(data): Json<SignIn>,
) -> Result<Json<Vec<i64>>, StatusCode> {
    let res = ctx.user_service.first_or_create(data.email.as_str())
        .await;
    match res {
        Ok(t) => {
            let res = ctx.subscription_service.get_subscriptions(t.0.id)
                .await;
            match res {
                Ok(vec) => {
                    Ok(Json(vec))
                }
                Err(e) => {
                    eprintln!("error handling sign in: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            eprintln!("error handling sign in: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn list_subscriptions(
    ctx: State<Context>,
    Path(uid): Path<i64>,
) -> Result<Json<Vec<i64>>, StatusCode> {
    let res = ctx.subscription_service.get_subscriptions(uid)
        .await;
    match res {
        Ok(vec) => { Ok(Json(vec)) }
        Err(e) => {
            eprintln!("error handling fetch subscriptions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}