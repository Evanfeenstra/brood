use yew::{html, Component, Properties, ComponentLink, Html, ShouldRender};

use crate::app::{Coin};

pub struct Page {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
}

struct State {
    receive_address: String,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String,
    pub color: String,
    pub balance: u64,
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
            props,
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
            <div class="page">
                <div class="page-name">{&self.props.name}</div>
                <div class="page-balance">
                    {"Balance:  "}
                    {&self.props.balance}
                </div>
                {self.view_send()}
            </div>
        }
    }
}

impl Page {

pub fn view_send(&self) -> Html {
    html! {
        <div class="send">{"SEND"}</div>
    }
}

}
