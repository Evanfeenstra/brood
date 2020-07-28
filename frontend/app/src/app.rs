use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::fetch::FetchTask;

use crate::components::{logo::Logo, grid::Grid, line::Line};

const KEY: &str = "yew.brood.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    fetcher: Option<FetchTask>,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    coins: Vec<Coin>,
    value: String,
    initted: bool,
    shimmer_url: String,
    url_input_value: String,
}

#[derive(Serialize, Deserialize)]
struct Coin {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Mint,
    ShowIcon,
    UpdateURL(String),
    EnterURL,
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
            initted: false,
            shimmer_url: "".to_string(),
            url_input_value: "".to_string(),
        };
        App {
            link,
            storage,
            state,
            fetcher: None,
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
            Msg::UpdateURL(val) => {
                self.state.url_input_value = val;
            }
            Msg::EnterURL => {
                info!("Enter!");
                self.state.shimmer_url = self.state.url_input_value.clone();
                self.state.url_input_value = "".to_string();
            }
            Msg::ShowIcon => {
                self.state.initted = true;
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.coins));
        true
    }

    fn view(&self) -> Html {
        html! {
            <main class="wrapper">
                <Grid done=self.link.callback(|_| Msg::ShowIcon) />
                <div class="app">
                    <section class="sidebar">
                        <header class=if self.state.initted {"sidebar-head"} else {"sidebar-head hide"}>
                            <Logo />
                            <div class="title">{"brood wallet"}</div>
                        </header>
                        <div class="sidebar-body">
                            {self.view_coins()}
                        </div>
                    </section>
                    <section class="content">
                        {self.view_content()}
                    </section>
                </div>
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
    fn view_content(&self) -> Html {
        if !self.state.initted {
            return html! {}
        }
        if self.state.shimmer_url.len()==0 {
            return self.view_url_input()
        }
        html! {}
    }
    fn view_url_input(&self) -> Html {
        html! {
            <div class="url-input-wrap">
                <input class="url-input"
                    placeholder="Input your Shimmer URL"
                    value=&self.state.url_input_value
                    oninput=self.link.callback(|e: InputData| Msg::UpdateURL(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::EnterURL } else { Msg::Nope }
                    })
                />
                <button class="url-input-button"
                    disabled=self.state.url_input_value.len()==0
                    onclick=self.link.callback(|_| Msg::EnterURL)
                >
                    {"OK"}
                </button>
            </div>
        }
    }
}

impl State {
    fn _total(&self) -> usize {
        self.coins.len()
    }
}
