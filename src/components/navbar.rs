use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

use crate::routes::{AppRoute, AppAnchor};

pub struct NavBar;

impl Component for NavBar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavBar {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html!(
            <nav class=classes!("container", "mx-auto", "text-left", "py-6", "font-mono")>
              <div class=classes!("w-full", "block", "flex-grow", "lg:flex", "lg:items-center", "lg:w-auto")>
                <div class=classes!("text-sm", "lg:flex-grow")>
                  <AppAnchor
                    classes="block mt-4 lg:inline-block lg:mt-0 hover:text-gray-500 mr-4"
                    route=AppRoute::Home
                  >
                      { "Home | " }
                  </AppAnchor>
                  <AppAnchor
                    classes="block mt-4 lg:inline-block lg:mt-0 hover:text-gray-500 mr-4"
                    route=AppRoute::About
                  >
                      { "About | " }
                  </AppAnchor>
                  <AppAnchor
                    classes="block mt-4 lg:inline-block lg:mt-0 hover:text-gray-500 mr-4"
                    route=AppRoute::Resume
                  >
                      { "Resume" }
                  </AppAnchor>
                </div>
              </div>
            </nav>
        )
    }
}
