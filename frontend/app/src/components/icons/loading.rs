use yew::{html, Component, ComponentLink, Properties, Html, ShouldRender};

pub struct Loading {
    size: String,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub size: String,
}

impl Component for Loading {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Loading {size:props.size}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.size = props.size;
        true
    }

    fn view(&self) -> Html {
        let mut c = "loading";
        if self.size=="small" {
            c = "loading small"
        }
        if self.size=="big" {
            c = "loading big"
        }
        html! {
            <svg fill="#FFFFFF" 
                class=c
                viewBox="0 0 1024 1024">
                <path d="M988 548c-19.9 0-36-16.1-36-36 0-59.4-11.6-117-34.6-171.3a440.45 440.45 0 0 0-94.3-139.9 437.71 437.71 0 0 0-139.9-94.3C629 83.6 571.4 72 512 72c-19.9 0-36-16.1-36-36s16.1-36 36-36c69.1 0 136.2 13.5 199.3 40.3C772.3 66 827 103 874 150c47 47 83.9 101.8 109.7 162.7 26.7 63.1 40.2 130.2 40.2 199.3.1 19.9-16 36-35.9 36z"/>
            </svg>
        }
    }
}