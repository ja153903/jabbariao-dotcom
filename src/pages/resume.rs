use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Resume;

impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Resume {}
    }

    fn view(&self) -> Html {
        html!(
            <div class="container">
                <h1>{ "Resume" }</h1>
                <p>{ "Talking about stuff I do here" }</p>
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
