#![allow(dead_code)]

use yew::prelude::*;
use yew::Html;

mod routes;
mod pages;

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
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"Stuff is going to happen here"}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
