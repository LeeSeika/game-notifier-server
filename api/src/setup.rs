use std::env;
use axum::Router;
use axum::routing::get;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use service::subscription::subscription::SubscriptionService;
use service::user::user::UserService;
use crate::handlers::test_wechat;

#[tokio::main]
pub(crate) async fn start() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.unwrap();

    let ctx = Context {
        user_service: UserService::new(&conn),
        subscription_service: SubscriptionService::new(&conn),
    };

    let app = Router::new()
        .route("/", get(test_wechat))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) user_service: UserService,
    pub(crate) subscription_service: SubscriptionService,
}