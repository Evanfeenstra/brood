use yew::{html, Component, Properties, ComponentLink, Html, ShouldRender};

use crate::app::{Coin};

pub struct Page {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
    receive_address: String,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub coin: Coin,
}

pub enum Msg {}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State{
            receive_address: "".to_string(),
        };
        Page {
            link,
            state,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>{"coin"}</div>
        }
    }
}
