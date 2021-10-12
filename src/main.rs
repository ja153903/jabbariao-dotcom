use yew::prelude::*;
use yew::Html;
use yew_router::switch::Permissive;

mod pages;
use pages::{about::About, home::Home, page_not_found::PageNotFound, resume::Resume};

mod routes;
use routes::{AppRoute, AppRouter};

struct Model;

impl Component for Model {
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
                   <AppRouter
                        render=AppRouter::render(Self::switch)
                   />
                </main>
            </>
        }
    }
}

impl Model {
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
    yew::start_app::<Model>();
}
