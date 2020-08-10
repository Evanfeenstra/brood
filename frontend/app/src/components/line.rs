use yew::{html, Properties, Component, ComponentLink, Html, ShouldRender};

pub struct Line {
    props: Props,
}

pub enum Msg {
    Fire,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

impl Component for Line {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Line {props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <line class="line" 
                x1=self.props.x1 
                y1=self.props.y1 
                x2=self.props.x2
                y2=self.props.y2
            />
        }
    }
}