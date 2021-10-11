#![allow(dead_code)]

use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PageNotFoundProps {
    pub route: Option<String>,
}

pub struct PageNotFound {
    props: PageNotFoundProps,
}

impl Component for PageNotFound {
    type Message = ();
    type Properties = PageNotFoundProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1 class="title">
                    { "Page not found" }
                </h1>
            </div>
        }
    }
}
