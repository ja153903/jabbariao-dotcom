use yew::prelude::*;
use yew::Html;
use yew_router::switch::Permissive;

mod pages;
use pages::{about::About, home::Home, page_not_found::PageNotFound, resume::Resume};

mod components;
use components::navbar::NavBar;

mod routes;
use routes::{AppRoute, AppRouter};

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <main>
                   <NavBar />
                   <AppRouter
                        render=AppRouter::render(Self::switch)
                   />
                </main>
            </>
        }
    }
}

impl App {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::About => html! { <About /> },
            AppRoute::Resume => html! { <Resume /> },
            AppRoute::Home => html! { <Home /> },
            AppRoute::PageNotFound(Permissive(route)) => html! { <PageNotFound route=route /> },
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
