use yew::{html, Component, Properties, Callback, ComponentLink, Html, ShouldRender};

pub struct Wallet {
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

impl Component for Wallet {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Wallet {
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
            return html! {
                <svg onclick=self.link.callback(|_| Msg::Clicked) class="wallet" fill="#FFFFFF" viewBox="0 0 1024 1024">
                    <path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-32 464H528V448h320v128zm-268-64a40 40 0 1 0 80 0 40 40 0 1 0-80 0z"/>
                </svg>
            }
        }
        html! {
            <svg onclick=self.link.callback(|_| Msg::Clicked) class="wallet" fill="#FFFFFF" viewBox="0 0 1024 1024">
                <path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-40 464H528V448h312v128zm0 264H184V184h656v200H496c-17.7 0-32 14.3-32 32v192c0 17.7 14.3 32 32 32h344v200zM580 512a40 40 0 1 0 80 0 40 40 0 1 0-80 0z"/>
            </svg>
        } 
    }
}
