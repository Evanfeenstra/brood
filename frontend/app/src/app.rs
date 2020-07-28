use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    storage::{Area, StorageService},
    fetch::{FetchTask, FetchService, Request, Response},
};
use yew::format::{Text, Nothing};
use anyhow::Error;

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
    fetching: bool,
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
    FetchReady(Result<DataFromAPI, Error>),
    Nope,
}

/// have to correspond the data layout from that file.
#[derive(Deserialize, Debug)]
pub struct DataFromAPI {
    value: u32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let persisted_url:String = {
            if let Json(Ok(persisted)) = storage.restore(KEY) {
                persisted
            } else {
                "".to_string()
            }
        };
        let state = State {
            coins: Vec::new(),
            value: "".to_string(),
            initted: false,
            shimmer_url: persisted_url,
            url_input_value: "".to_string(),
            fetching: false,
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
                self.state.fetching = true;
                self.fetch_json(
                    self.state.shimmer_url.clone() + "/check"
                );   
            }
            Msg::ShowIcon => {
                self.state.initted = true;
            }
            Msg::FetchReady(response) => {
                self.state.fetching = false;
                info!("FETCH DON!")
                // self.data = response.map(|data| data.value).ok();
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.shimmer_url));
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
    fn fetch_json(&mut self, path: String) {
        let callback = self.link.callback(
            move |response: Response<Json<Result<DataFromAPI, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    info!("Error Bad Request");
                    Msg::Nope // FIXME: Handle this error accordingly.
                }
            },
        );
        match Request::get(path).body(Nothing) {
            Ok(req) => {
                let res = FetchService::fetch(req, callback);
                self.fetcher = Some(res.unwrap());
            },
            Err(e) => () // handle error here
        };
    }
}

impl State {
    fn _total(&self) -> usize {
        self.coins.len()
    }
}
