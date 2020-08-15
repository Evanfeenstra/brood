use yew::{html, Component, Properties, Callback, ComponentLink, Html, ShouldRender};

use crate::components::icons::{loading::Loading};

pub struct Plus {
    link: ComponentLink<Self>,
    active: bool,
    loading: bool,
    onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub loading: bool,
    pub onclick: Callback<()>,
}

impl Component for Plus {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Plus {
            link,
            active: props.active,
            loading: props.loading,
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
        self.loading = props.loading;
        true
    }

    fn view(&self) -> Html {
        if self.active {
            return html! {
                <svg onclick=self.link.callback(|_| Msg::Clicked) class="plus" fill="#FFFFFF" viewBox="0 0 1024 1024">
                    <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm192 472c0 4.4-3.6 8-8 8H544v152c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V544H328c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h152V328c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v152h152c4.4 0 8 3.6 8 8v48z"/>
                </svg>
            }
        }
        if self.loading {
            return html!{<Loading size="" />}
        }
        html! {
            <svg onclick=self.link.callback(|_| Msg::Clicked) class="plus" fill="#FFFFFF" viewBox="0 0 1024 1024">
                <path d="M696 480H544V328c0-4.4-3.6-8-8-8h-48c-4.4 0-8 3.6-8 8v152H328c-4.4 0-8 3.6-8 8v48c0 4.4 3.6 8 8 8h152v152c0 4.4 3.6 8 8 8h48c4.4 0 8-3.6 8-8V544h152c4.4 0 8-3.6 8-8v-48c0-4.4-3.6-8-8-8z"/>
                <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z"/>
            </svg>
        } 
    }
}


