#![allow(dead_code)]

use yew::prelude::*;
use yew::Html;
use yew_router::route::Route;
use yew_router::switch::Permissive;

mod pages;
use pages::{about::About, page_not_found::PageNotFound};

mod routes;
use routes::{AppRoute, AppRouter, PublicUrlSwitch};

enum Msg {}

struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                   />
                </main>
            </>
        }
    }
}

impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            // TODO: Create basic component for Home
            AppRoute::Home => {
                html! { <About /> }
            }
            AppRoute::About => {
                html! { <About /> }
            }
            // TODO: Create basic component for Resume
            AppRoute::Resume => {
                html! { <About /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
