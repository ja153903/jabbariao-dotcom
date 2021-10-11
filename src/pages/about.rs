use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        About {}
    }

    fn view(&self) -> Html {
        html!(
            <div class="container">
                <h1>{ "About" }</h1>
                <p>{ "I am Jaime Abbariao. A software engineer at BentoBox" }</p>
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
