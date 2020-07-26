use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

use crate::components::{logo::Logo, grid::Grid, line::Line};

const KEY: &str = "yew.brood.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    coins: Vec<Coin>,
    value: String,
    show_icon: bool,
}

#[derive(Serialize, Deserialize)]
struct Coin {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Mint,
    Update(String),
    ShowIcon,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let coins = {
            if let Json(Ok(restored_coins)) = storage.restore(KEY) {
                restored_coins
            } else {
                Vec::new()
            }
        };
        let state = State {
            coins,
            value: "".to_string(),
            show_icon: false,
        };
        App {
            link,
            storage,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Mint => {
                let coin = Coin {
                    description: self.state.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.state.coins.push(coin);
                self.state.value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::ShowIcon => {
                info!("DONEEE");
                self.state.show_icon = true;
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.coins));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <main class="wrapper">
                <div class="app">
                    <section class="sidebar">
                        <header class=if self.state.show_icon {"sidebar-head"} else {"sidebar-head hide"}>
                            <Logo />
                            <div class="title">{"brood wallet"}</div>
                        </header>
                        <div class="sidebar-body">
                            {self.view_coins()}
                        </div>
                    </section>
                </div>
                <Grid done=self.link.callback(|_| Msg::ShowIcon) />
            </main>
        }
    }
}

impl App {
    fn view_coins(&self) -> Html {
        html! {
            <div>{"."}</div>
        }
    }
}

impl State {
    fn _total(&self) -> usize {
        self.coins.len()
    }
}
