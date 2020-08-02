use yew::{html, Component, ComponentLink, Properties, Callback, Html, ShouldRender};

use crate::components::icons::{loading::Loading};

pub struct Faucet {
    link: ComponentLink<Self>,
    active: bool,
    onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub onclick: Callback<()>,
}

impl Component for Faucet {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Faucet {
            link,
            active: props.active,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onclick.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.active = props.active;
        true
    }

    fn view(&self) -> Html {
        if self.active {
            return html!{<Loading big=false />}
        }
        html! {
<svg class="faucet" width="28" height="28" viewBox="0 0 490 490" fill="white"
    onclick=self.link.callback(|_| Msg::Clicked)>
    <g>
        <path d="M469.2,95.7H354.8V41.6h28.1c11.4,0,20.8-9.4,20.8-20.8S394.3,0,382.9,0h-97.8c-11.4,0-20.8,9.4-20.8,20.8
            s9.4,20.8,20.8,20.8h28.1v54.1H90.5C40.6,95.7,0,135.2,0,184.1v158.1C0,353.6,9.4,363,20.8,363h139.4c11.4,0,20.8-8.3,20.8-19.8
            v-69.7h288.2c11.4,0,20.8-9.4,20.8-20.8V116.5C490,105.1,480.6,95.7,469.2,95.7z M449.4,232H161.3c-11.4,0-20.8,9.4-20.8,20.8
            v69.7H40.6V184.1c0-26,22.9-47.9,49.9-47.9h358.9V232z"/>
        <path d="M25,380.8c-11.4,0-20.8,9.4-20.8,20.8v67.6c0,11.4,9.4,20.8,20.8,20.8s20.8-9.4,20.8-20.8v-67.6
            C45.8,390.1,36.4,380.8,25,380.8z"/>
        <path d="M85.3,380.8c-11.4,0-20.8,9.4-20.8,20.8v67.6c0,11.4,9.4,20.8,20.8,20.8s20.8-9.4,20.8-20.8v-67.6
            C106.1,390.1,96.8,380.8,85.3,380.8z"/>
        <path d="M145.6,380.8c-11.4,0-20.8,9.4-20.8,20.8v67.6c0,11.4,9.4,20.8,20.8,20.8c11.4,0,20.8-9.4,20.8-20.8v-67.6
            C166.5,390.1,157.1,380.8,145.6,380.8z"/>
    </g>
</svg>
        }
    }
}

