use yew::{html, Properties, Component, ComponentLink, Callback, Html, ShouldRender};
use yew::services::{Task,RenderService,ResizeService,resize};
// use log::*;

use crate::components::{line::Line};

pub struct Grid {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
    speed: i16,
    render_loop: Option<Box<dyn Task>>,
    resize_task: Option<resize::ResizeTask>,
    points: Vec<(i16,i16)>,
}

pub enum Msg {
    Init(f64),
    Render(f64),
    Resize(resize::WindowDimensions),
    Nope,
}

pub struct State {
    height: i16,
    width: i16,
    l: i16,
    total: i16,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub done: Callback<()>,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State{
            height: 0,
            width: 0,
            l: 0,
            total: 0,
        };
        Grid {
            link,
            state,
            props,
            render_loop: None,
            resize_task: None,
            speed: 120,
            points: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(_) => {
                self.render_gl();
            }
            Msg::Init(_) => {
                self.init();
            }
            Msg::Resize(val) => {
                let w = val.width as i16;
                let h = val.height as i16;
                self.set_size(w,h);
            }
            Msg::Nope => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let render_frame = self.link.callback(Msg::Init);
            let handle = RenderService::request_animation_frame(render_frame);
            self.render_loop = Some(Box::new(handle)); // store or its dropped

            let resize = self.link.callback(Msg::Resize);
            let resize_handle = ResizeService::new().register(resize);
            self.resize_task = Some(resize_handle);
        }
    }

    fn view(&self) -> Html {
        // info!("rendered! {}", self.state.l);
        let points = self.points.clone();
        let mut offset = 0;
        let l = self.state.l;
        let mut view_line = |(i, p): (usize, &(i16,i16))| {
            if l<offset || i==points.len()-1 {
                return html!{}
            }
            let next = points[i+1];
            let len = self.calc_length(p.0,p.1,next.0,next.1);

            let x1 = p.0;
            let y1 = p.1;
            let mut x2 = next.0;
            let mut y2 = next.1;

            let start = l-offset;
            if y1>y2 { // up
                y2 = bigger_of(next.1, next.1 + len - start);
            }
            if y1<y2 { // down
                y2 = smaller_of(next.1, next.1 - len + start);
            }
            if x1>x2 { // left
                x2 = bigger_of(next.0, next.0 + len - start);
            }
            if x1<x2 { // right
                x2 = smaller_of(next.0, next.0 - len + start);
            }

            offset += len;
            
            html! {
                <Line 
                    x1=x1 as u16
                    y1=y1 as u16  
                    x2=x2 as u16  
                    y2=y2 as u16  
                />
            }
        };

        let total = self.state.total;

        let start_final = total - 54;
        let go_final = bigger_of(2, l-start_final);
        let final_y2 = bigger_of(2, 56-go_final);
        fn view_final_line(fy2:i16,go:i16)->Html {
            if go<=2 {
                return html!{}
            }
            html!{
                <Line 
                    x1=280 y1=56
                    x2=280 y2=fy2 as u16
                />
            }
        }

        html! {
            <svg class="grid" 
                height=&self.state.height width=&self.state.width
            >
                {view_final_line(final_y2, go_final)}
                {for points.iter().enumerate() // enumerate index
                    .map(|p| view_line(p))
                }
            </svg>
        }
    }
}

impl Grid {
    fn set_size(&mut self, w:i16, h:i16) {
        self.state.width = w;
        self.state.height = h;
        self.points = self.make_points(w-2, h-2);
        self.state.total = self.calc_total(self.points.clone());
    }
    fn make_points(&mut self, w:i16, h:i16) -> Vec<(i16,i16)> {
        vec![
            (280,h),
            (280,56),
            (2,56),
            (2,h),
            (w,h),
            (w,2),
            (2,2),
            (2,56),
        ]
    }
    fn render_gl(&mut self) {
        self.state.l = self.state.l + self.speed;
        if self.state.l > self.state.total * 2 {
            self.props.done.emit(());
            return // stop the loop
        }
        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);
        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(Box::new(handle));
    }
    fn init(&mut self) {
        let window = web_sys::window().unwrap();
        let w = match window.inner_width().unwrap().as_f64() {
            Some(jwidth) => jwidth as i16,
            _ => 0,
        };
        let h = match window.inner_height().unwrap().as_f64() {
            Some(jheight) => jheight as i16,
            _ => 0,
        };
        self.set_size(w, h);
        self.render_gl();
    }
    fn calc_total(&mut self, points:Vec<(i16,i16)>) -> i16 {
        points.iter().enumerate().fold(0, |sum, (i,p):(usize, &(i16,i16))| {
            let mut next = p; // so no len
            if i<self.points.len()-1 {
                next = &self.points[i+1];
            }
            return sum + self.calc_length(p.0,p.1,next.0,next.1);
        })
    }
    fn calc_length(&self, x1:i16, y1:i16, x2:i16, y2:i16) ->i16 {
        let xlen = x1 - x2;
        let ylen = y1 - y2;
        (xlen.abs() - ylen.abs()).abs()
    }
}

fn smaller_of(n1:i16, n2:i16)->i16 {
    if n1<n2 {
        return n1
    }
    n2
}
fn bigger_of(n1:i16, n2:i16)->i16 {
    if n1>n2 {
        return n1
    }
    n2
}