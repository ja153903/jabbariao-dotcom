use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        About {}
    }

    fn view(&self) -> Html {
        html!(
            <div class=classes!("container", "mx-auto", "text-left", "font-mono")>
                <h2 class=classes!("text-2xl", "py-8")>{ "About" }</h2>
                <div>
                    <p class=classes!("py-2")>{ "I am Jaime Abbariao" }</p>
                    <p class=classes!("py-2")>
                        { "I write code for " }
                        <a href="https://getbento.com" target="_blank" class=classes!("text-gray-700", "hover:underline")>{ "BentoBox" }</a>
                    </p>
                    <p class=classes!("py-2")>
                        { "I create photographs. Check them out at " }
                        <a href="https://www.instagram.com/jaimeabbariao/" target="_blank" class=classes!("text-gray-700", "hover:underline")>
                            { "@jaimeabbariao" }
                        </a>
                    </p>
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
