use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/resume"]
    Resume,
    #[to = "/"]
    Home,
}

// TODO: Create routing strategy so we can actually switch between routes
