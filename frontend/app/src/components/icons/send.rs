use yew::{html, Component, ComponentLink, Properties, Html, ShouldRender};

use crate::components::icons::{loading::Loading};

pub struct Send {
    link: ComponentLink<Self>,
    active: bool,
}

pub enum Msg {
    Clicked,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
}

impl Component for Send {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Send {
            link,
            active: props.active,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
<svg class="send-icon" width="28" height="28" viewBox="0 0 1024 1024">
    <path d="M931.4 498.9L94.9 79.5c-3.4-1.7-7.3-2.1-11-1.2-8.5 2.1-13.8 10.7-11.7 19.3l86.2 352.2c1.3 5.3 5.2 9.6 10.4 11.3l147.7 50.7-147.6 50.7c-5.2 1.8-9.1 6-10.3 11.3L72.2 926.5c-0.9 3.7-0.5 7.6 1.2 10.9 3.9 7.9 13.5 11.1 21.5 7.2l836.5-417c3.1-1.5 5.6-4.1 7.2-7.1 3.9-8 0.7-17.6-7.2-21.6zM170.8 826.3l50.3-205.6 295.2-101.3c2.3-0.8 4.2-2.6 5-5 1.4-4.2-0.8-8.7-5-10.2L221.1 403 171 198.2l628 314.9-628.2 313.2z" />
</svg>
        }
    }
}

