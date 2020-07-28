use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Gear {}

pub enum Msg {}

impl Component for Gear {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Gear {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {


        }
    }
}