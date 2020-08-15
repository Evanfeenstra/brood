use yew::{html, Component, ComponentLink, Callback, Properties, Html, ShouldRender};

pub struct Edit {
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

impl Component for Edit {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Edit {
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
            return html!{<svg class="edit-icon" viewBox="0 0 1024 1024" fill="white"
                onclick=self.link.callback(|_| Msg::Clicked)>
                <path d="M257.7,752c2,0,4-0.2,6-0.5L431.9,722c2-0.4,3.9-1.3,5.3-2.8l423.9-423.9c3.9-3.9,3.9-10.2,0-14.1L694.9,114.9
                    c-1.9-1.9-4.4-2.9-7.1-2.9s-5.2,1-7.1,2.9L256.8,538.8c-1.5,1.5-2.4,3.3-2.8,5.3l-29.5,168.2c-1.9,11.1,1.5,21.9,9.4,29.8
                    C240.5,748.5,248.8,752,257.7,752z"
                />
            </svg>}
        }
        html! {<svg class="edit-icon" viewBox="0 0 1024 1024" fill="white"
            onclick=self.link.callback(|_| Msg::Clicked)>
            <path d="M257.7,752c2,0,4-0.2,6-0.5L431.9,722c2-0.4,3.9-1.3,5.3-2.8l423.9-423.9c3.9-3.9,3.9-10.2,0-14.1L694.9,114.9
		        c-1.9-1.9-4.4-2.9-7.1-2.9s-5.2,1-7.1,2.9L256.8,538.8c-1.5,1.5-2.4,3.3-2.8,5.3l-29.5,168.2c-1.9,11.1,1.5,21.9,9.4,29.8
                C240.5,748.5,248.8,752,257.7,752z M325.1,577.6L687.8,215l73.3,73.3L398.4,650.9l-88.9,15.7L325.1,577.6z"
            />
        </svg>}
    }
}

