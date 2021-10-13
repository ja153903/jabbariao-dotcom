use yew::{html, classes, Component, ComponentLink, Html, ShouldRender};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn view(&self) -> Html {
        html!(
            <div class=classes!("container", "mx-auto", "text-left", "font-mono")>
                <h2 class=classes!("text-2xl", "py-8")>{ "What am I currently working on?" }</h2>
                <p>{ "Diving into Rust, WASM, and Cryptography" }</p>
            </div>
        )
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
}
