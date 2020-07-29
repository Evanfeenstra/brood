use log::{info,warn};
use serde_derive::{Deserialize, Serialize};
use serde_json::{
    json,
    value::Value,
    error::Error,
    from_str,
};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    storage::{Area, StorageService},
    fetch::{FetchTask, FetchService, Request, Response},
};
use yew::format::{Text, Nothing};
use anyhow::{anyhow};

use crate::components::{logo::Logo, grid::Grid, gear::Gear, loading::Loading};

const KEY: &str = "yew.brood.self.shimmer_url";

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
    synced: bool,
    version: String,
    seed: String,
    identity_id: String,
    settings_active: bool,
    has_wallet: bool,
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
    FetchReady(&'static str, String),
    FetchErr(anyhow::Error),
    SettingsClicked,
    Create,
    SeedCopied,
    Nope,
}

/// have to correspond the data layout from that endpoint.
#[derive(Deserialize, Debug)]
pub struct CheckRes {
    version: String,
    synced: bool,
    identity_id: String,
    has_wallet: bool,
}
#[derive(Deserialize, Debug)]
pub struct CreateRes {
    seed: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State {
            initted: false,
            coins: Vec::new(),
            value: "".to_string(),
            url_input_value: "".to_string(),
            fetching: false,
            synced: false,
            version: "".to_string(),
            seed: "".to_string(),
            identity_id: "".to_string(),
            settings_active: false,
            has_wallet: false,
            shimmer_url: {
                if let Json(Ok(persisted)) = storage.restore(KEY) {
                    persisted
                } else {
                    "".to_string()
                }
            }
        };
        let mut app = App {
            link,
            storage,
            state,
            fetcher: None,
        };
        if app.state.shimmer_url.len()>0 {
            app.fetch_json("check", json!({
                "url": app.state.shimmer_url
            }));
        }
        app
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateURL(val) => {
                self.state.url_input_value = val;
            }
            Msg::EnterURL => {
                let mut val = self.state.url_input_value.clone();
                if val.chars().last()==Some('/') {
                    val.pop();
                }
                info!("url: {:?}",val);
                self.state.shimmer_url = val;
                self.state.url_input_value = "".to_string();
                self.state.fetching = true;
                self.fetch_json("check", json!({
                    "url": self.state.shimmer_url
                }));   
            }
            Msg::ShowIcon => {
                self.state.initted = true;
            }
            Msg::FetchReady(path, data) => {
                self.state.fetching = false;
                self.parse_json_response(path, data);
                info!("shimmer synced: {:?}",self.state.synced);
                info!("has_wallet {:?}",self.state.has_wallet);
            }
            Msg::FetchErr(err) => {
                warn!("{:?}",err)
            }
            Msg::Create => {
                self.fetch_json("create", json!({
                    "url": self.state.shimmer_url
                })); 
            }
            Msg::SeedCopied => {
                self.state.seed = "".to_string();
                self.state.has_wallet = true;
            }
            Msg::SettingsClicked => {
                self.state.settings_active = !self.state.settings_active;
            }
            Msg::Mint => {
                let coin = Coin {
                    description: self.state.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.state.coins.push(coin);
                self.state.value = "".to_string();
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
                    {self.view_content()}
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
        let mut synced_text = "NOT SYNCED";
        if self.state.synced {
            synced_text = "SYNCED";
        }
        html! {
            <section class="content">
                <header class="content-header">
                    <div /> // empty div for flex
                    <div class="node-info">
                        <div>{synced_text}</div>
                        <div>{&self.state.identity_id}</div>
                        <div>{&self.state.version}</div>
                        <Gear active=self.state.settings_active 
                            onclick=self.link.callback(|_| Msg::SettingsClicked)
                        />
                    </div>
                </header>
                <div class="content-body">
                    {self.view_body()}
                </div>
            </section>
        }
    }
    fn view_body(&self) -> Html {
        if !self.state.has_wallet && self.state.seed.len()==0 {
            return html!{
                <div class="create">
                    <p>{"Create your wallet!"}</p>
                    <button class="button create-button"
                        onclick=self.link.callback(|_| Msg::Create)
                    >{"CREATE NOW"}</button>
                </div>
            }
        }
        if !self.state.has_wallet {
            return html!{
                <div class="show-seed">
                    <p>{"Copy and save your seed. It will not be show again!"}</p>
                    <pre>{&self.state.seed}</pre>
                    <button class="button seed-button"
                        onclick=self.link.callback(|_| Msg::SeedCopied)
                    />
                </div>
            }
        }
        html!{
            
        }
    }
    fn view_url_input(&self) -> Html {
        html! {
            <section class="content-center">
                <div class="url-input-wrap">
                    <input class="url-input"
                        placeholder="Input your Shimmer URL"
                        value=&self.state.url_input_value
                        oninput=self.link.callback(|e: InputData| Msg::UpdateURL(e.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { Msg::EnterURL } else { Msg::Nope }
                        })
                    />
                    <button class="button url-input-button"
                        disabled=self.state.url_input_value.len()==0
                        onclick=self.link.callback(|_| Msg::EnterURL)
                    >
                        {"OK"}
                    </button>
                </div>
            </section>
        }
    }
    fn fetch_json(&mut self, path:&'static str, body: Value) {
        let callback = self.link.callback(
            move |response: Response<Text>| {
                let (meta, data) = response.into_parts();
                if meta.status.is_success() {
                    match data {
                        Ok(d)=> Msg::FetchReady(path, d),
                        Err(e)=> Msg::FetchErr(e),
                    }
                } else {
                    Msg::FetchErr(anyhow!("cant fetch"))
                }
            },
        );
        match Request::post("http://localhost:3579/".to_string()+&path).body(Json(&body)) {
            Ok(req) => {
                let res = FetchService::fetch(req, callback);
                self.fetcher = Some(res.unwrap());
            },
            Err(e) => { Msg::FetchErr(anyhow::Error::new(e)); }
        };
    }
    fn parse_json_response(&mut self, path:&'static str, r:String){
        match path {
            "check"=> {
                let json: Result<CheckRes,Error> = from_str(r.as_str());
                json.map(|data| {
                    self.state.synced = data.synced;
                    self.state.version = data.version;
                    self.state.identity_id = data.identity_id;
                    self.state.has_wallet = data.has_wallet;
                }).ok();
            }
            "create"=>{
                let json: Result<CreateRes,Error> = from_str(r.as_str());
                json.map(|data| {
                    self.state.seed = data.seed;
                }).ok();
            }
            &_=>()
        }
    }
}

impl State {
    fn _total(&self) -> usize {
        self.coins.len()
    }
}
