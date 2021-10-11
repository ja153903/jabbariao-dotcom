#![allow(dead_code)]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum AboutMessage {}

pub struct About {
    link: ComponentLink<Self>,
}

impl Component for About {
    type Message = AboutMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        About { link }
    }

    fn view(&self) -> Html {
        html!(
            <div>
                <h1>{"About"}</h1>
                <p>{"I am Jaime Abbariao. A software engineer at BentoBox"}</p>
            </div>
        )
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }
}
