use serde_derive::{Deserialize, Serialize};
use serde_json::{json};
use std::time::Duration;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    Task, TimeoutService, IntervalService,
    storage::{Area, StorageService},
    fetch::{FetchTask},
};
use std::collections::HashMap;
use crate::utils::valid;
use crate::utils::host;
use crate::map;

const KEY: &str = "brood.shimmer_url";
const IOTA_COLOR: &str = "IOTA";

pub struct App {
    pub link: ComponentLink<Self>,
    pub storage: StorageService,
    pub state: State,
    pub fetcher: Option<FetchTask>,
    pub timeout: Option<Box<dyn Task>>,
    pub timeout_callback: Callback<()>,
    pub interval: Option<Box<dyn Task>>,
    pub interval_callback: Callback<()>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub coins: Vec<Coin>,
    pub initted: bool,
    pub shimmer_url: String,
    pub url_input_value: String,
    pub checking: bool,
    pub synced: bool,
    pub version: String,
    pub seed: String,
    pub identity_id: String,
    pub settings_active: bool,
    pub receive_active: bool,
    pub has_wallet: bool,
    pub copied: bool,
    pub confirmed_balance: HashMap<String, u64>,
    pub pending_balance: HashMap<String, u64>,
    pub addresses: Vec<Address>,
    pub selected_color: String,
    pub creating: bool,
    pub changing_url: bool,
    pub interval_counter: u8,
    pub interval_level: u8,
    pub meta_key_down: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Properties)]
pub struct Coin {
    pub name: String,
    pub color: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub address: String,
    pub index: i64,
    pub is_spent: bool,
    pub is_receive: bool,
}

pub enum Msg {
    ShowLogo,
    UpdateURL(String),
    EnterURL,
    EnterChangedURL,
    FetchDone(&'static str, String),
    FetchErr(anyhow::Error),
    SettingsClicked,
    ReceiveClicked,
    Create,
    SeedCopied,
    AddressCopied(String),
    TimeoutDone,
    Interval,
    CoinClicked(String),
    Reload,
    CreateClicked,
    CoinCreated(Coin,u64),
    PencilClicked,
    InputKeyEvent(String,String,String),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let empty_iota = vec![Coin{
            name:IOTA_COLOR.to_string(),
            color:IOTA_COLOR.to_string(),
            symbol:"I".to_string(),
        }];
        let mut shimmer_url = {
            if let Json(Ok(persisted)) = storage.restore(KEY) {
                persisted
            } else {
                "".to_string()
            }
        };
        if host::is_shimmer_node() { // docker network
            shimmer_url = "goshimmer:8080".to_string();
        }
        if host::is_shimmer_node_no_docker() {
            shimmer_url = "http://localhost:8080".to_string();
        }
        // is_shimmer_node
        let state = State {
            initted: false,
            coins: empty_iota,
            confirmed_balance: map!{IOTA_COLOR.to_string()=>0},
            pending_balance: HashMap::new(),
            addresses: Vec::new(),
            url_input_value: "".to_string(),
            checking: false,
            synced: false,
            version: "".to_string(),
            seed: "".to_string(),
            identity_id: "".to_string(),
            settings_active: false,
            receive_active: false,
            has_wallet: false,
            copied: false,
            selected_color: "".to_string(),
            creating: false,
            changing_url: true,
            interval_counter: 0,
            interval_level: 3,
            meta_key_down: false,
            shimmer_url,
        };
        let mut app = App {
            link: link.clone(),
            storage,
            state,
            fetcher: None,
            timeout: None,
            timeout_callback: link.callback(|_| Msg::TimeoutDone),
            interval: None,
            interval_callback: link.callback(|_| Msg::Interval),
        };
        if app.state.shimmer_url.len()>0 {
            app.fetch_json("check", json!({
                "url": app.state.shimmer_url
            }));
        }
        let handle = IntervalService::spawn(
            Duration::from_secs(1), // drop the demical
            app.interval_callback.clone()
        );
        app.interval = Some(Box::new(handle));
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
            Msg::EnterURL=> {
                if self.state.checking {
                    return false;
                }
                let val = valid::process_ip(self.state.url_input_value.clone());
                // info!("url: {:?}",val);
                self.state.shimmer_url = val;
                self.state.url_input_value = "".to_string();
                
                self.state.checking = true;
                self.fetch_json("check", json!({
                    "url": self.state.shimmer_url
                }));
            }
            Msg::EnterChangedURL=> {
                if self.state.checking {
                    return false;
                }
                let val = valid::process_ip(self.state.url_input_value.clone());
                self.state.shimmer_url = val;

                self.state.synced = false;
                self.state.version = "".to_string();
                self.state.identity_id = "".to_string();

                self.state.checking = true;
                self.fetch_json("check", json!({
                    "url": self.state.shimmer_url
                }));
            }
            Msg::PencilClicked=> {
                if !self.state.changing_url {
                    self.state.url_input_value = self.state.shimmer_url.clone();
                } else {
                    self.state.url_input_value = "".to_string();
                }
                self.state.changing_url = !self.state.changing_url;
                
            }
            Msg::ShowLogo=> {
                self.state.initted = true;
            }
            Msg::FetchDone(path, data)=> {
                self.state.checking = false;
                self.parse_json_response(path, data);
                // wallet is there! load data               // reload balance
                if path=="check" && self.state.has_wallet {
                    self.fetch_json("state", json!({}));
                }
            }
            Msg::FetchErr(err)=> {
                log::warn!("{:?}",err);
                self.state.checking = false
            }
            Msg::Create=> {
                self.fetch_json("create", json!({
                    "url": self.state.shimmer_url
                })); 
            }
            Msg::SeedCopied=> {
                self.fetch_json("clipboard", json!({
                    "cmd": "copy",
                    "text": self.state.seed,
                    "meta": "seed",
                }));
                self.state.seed = "".to_string();
                self.state.has_wallet = true;
            }
            Msg::AddressCopied(addy)=> {
                self.fetch_json("clipboard", json!({
                    "cmd": "copy",
                    "text": addy,
                    "meta": "addy",
                }));
                self.state.copied = true;
                let handle = TimeoutService::spawn(Duration::from_secs(3), self.timeout_callback.clone());
                self.timeout = Some(Box::new(handle));
            }
            Msg::TimeoutDone=> {
                self.state.copied = false;
            }
            Msg::Interval=> {
                if !self.state.synced || !self.state.has_wallet {
                    return false;
                }
                if self.state.interval_level<3 {
                    self.state.interval_level=3;
                };
                self.state.interval_counter = self.state.interval_counter+1;
                if self.state.interval_counter >= self.state.interval_level {
                    let n:f64 = self.state.interval_level as f64 * 1.4;
                    if n<32.0 {
                        self.state.interval_level = n as u8;
                    }
                    self.state.interval_counter = 0;
                    self.fetch_json("state", json!({}));
                }
            }
            Msg::SettingsClicked=> {
                self.state.receive_active = false;
                self.state.changing_url = false;
                self.state.settings_active = !self.state.settings_active;
            }
            Msg::ReceiveClicked=> {
                self.state.settings_active = false;
                self.state.receive_active = !self.state.receive_active;
            }
            Msg::CoinClicked(color)=> {
                self.state.creating = false;
                if self.state.selected_color==color {
                    self.state.selected_color = "".to_string();
                } else {
                    self.state.selected_color=color;
                }
            }
            Msg::Reload=> {
                self.state.interval_level = 3;
                self.state.interval_counter = 0;
            }
            Msg::CreateClicked=> {
                self.state.selected_color = "".to_string();
                self.state.creating = !self.state.creating
            }
            Msg::CoinCreated(coin,_balance)=>{
                self.state.coins.push(coin);
                self.state.interval_level = 3;
                self.state.interval_counter = 0;
                self.state.creating = false;
            }
            Msg::InputKeyEvent(direction,key,field)=> {
                if key == "Meta" || key == "Control" {
                    self.state.meta_key_down = direction=="down";
                }
                if self.state.meta_key_down {
                    if key=="c" || key=="x" {
                        self.fetch_json("clipboard", json!({
                            "cmd": "copy",
                            "text": self.state.url_input_value,
                            "meta": field,
                        }));
                        if key=="x" {
                            self.state.url_input_value="".to_string();
                        }
                    }
                    if key=="v" {
                        self.fetch_json("clipboard", json!({
                            "cmd": "paste",
                            "text": self.state.url_input_value,
                            "meta": field,
                        }));
                    }
                }
            }
            Msg::Nope=> {}
        }
        self.storage.store(KEY, Json(&self.state.shimmer_url));
        // info!("{:?}",self.state);
        true
    }

    fn view(&self) -> Html {
        self.view_app()
    }
}
