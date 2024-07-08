use crate::subscription::subscription::SubscriptionService;
use crate::user::user::UserService;

pub mod dispatcher;
pub mod subscription;
pub mod user;


#[derive(Clone)]
pub struct Context {
    pub user_service: UserService,
    pub subscription_service: SubscriptionService,
}