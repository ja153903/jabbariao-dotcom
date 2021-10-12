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
            <div class=classes!("container", "mx-auto", "text-center", "py-4", "font-mono")>
                <h1 class=classes!("text-4xl")>{ "Home" }</h1>
                <div class=classes!("py-8")>
                    <h2 class=classes!("text-2xl")>{ "What am I currently working on?" }</h2>
                    <p>{ "Currently focused on Rust, Cryptography, and WASM" }</p>
                </div>
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
