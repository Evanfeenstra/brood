
use yew::{html, Component, Properties, Callback, ComponentLink, Html, ShouldRender};

pub struct Left {
    link: ComponentLink<Self>,
    onclick: Callback<()>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}

impl Component for Left {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Left {
            link,
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
<svg class="back-arrow" stroke="white" viewBox="0 0 1024 1024" fill="white"
    onclick=self.link.callback(|_| Msg::Clicked)>
    <path fill="none" d="M512,140c-205.4,0-372,166.6-372,372s166.6,372,372,372s372-166.6,372-372S717.4,140,512,140z M616,380.9
    c0,10.2-4.9,19.9-13.2,25.9L457.4,512l145.4,105.2c8.3,6,13.2,15.6,13.2,25.9V690c0,6.5-7.4,10.3-12.7,6.5l-246-178
    c-4.4-3.2-4.4-9.7,0-12.9l246-178c5.3-3.8,12.7-0.1,12.7,6.5V380.9z"/>
    <path d="M603.3,327.5l-246,178c-4.4,3.2-4.4,9.7,0,12.9l246,178c5.3,3.8,12.7,0,12.7-6.5v-46.9c0-10.2-4.9-19.9-13.2-25.9
    L457.4,512l145.4-105.2c8.3-6,13.2-15.6,13.2-25.9V334C616,327.5,608.6,323.7,603.3,327.5z"/>
    <path d="M512,64C264.6,64,64,264.6,64,512s200.6,448,448,448s448-200.6,448-448S759.4,64,512,64z M512,884
    c-205.4,0-372-166.6-372-372s166.6-372,372-372s372,166.6,372,372S717.4,884,512,884z"/>
</svg>
        }
    }
}

