use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod about;
pub mod home;
pub mod index;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
}
