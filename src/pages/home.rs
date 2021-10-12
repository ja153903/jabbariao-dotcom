use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn view(&self) -> Html {
        html!(
            <div class="container">
                <h1>{ "Home" }</h1>
                <p>{ "Currently focused on Rust, Cryptography, and WASM" }</p>
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
