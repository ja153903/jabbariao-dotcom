#![allow(dead_code)]

use yew_router::prelude::{Router, RouterAnchor};
use yew_router::switch::Permissive;
use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/resume"]
    Resume,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>; 
