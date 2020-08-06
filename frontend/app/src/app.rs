use log::{info,warn};
use serde_derive::{Deserialize, Serialize};
use serde_json::{json};
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    Task, TimeoutService,
    storage::{Area, StorageService},
    fetch::{FetchTask},
};
use std::collections::HashMap;

use crate::components::{grid::Grid};
use crate::components::icons::{logo::Logo};
use crate::utils::web;

const KEY: &str = "brood.shimmer_url";

pub struct App {
    pub link: ComponentLink<Self>,
    pub storage: StorageService,
    pub state: State,
    pub fetcher: Option<FetchTask>,
    pub timeout: Option<Box<dyn Task>>,
    pub timeout_callback: Callback<()>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub coins: Vec<Coin>,
    pub initted: bool,
    pub shimmer_url: String,
    pub url_input_value: String,
    pub fetching: bool,
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
}

#[derive(Serialize, Deserialize, Debug)]
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
    Mint,
    ShowIcon,
    UpdateURL(String),
    EnterURL,
    FetchDone(&'static str, String),
    FetchErr(anyhow::Error),
    SettingsClicked,
    ReceiveClicked,
    Create,
    SeedCopied,
    AddressCopied(String),
    Balance,
    TimeoutDone,
    FaucetClicked,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let state = State {
            initted: false,
            coins: Vec::new(),
            url_input_value: "".to_string(),
            fetching: false,
            synced: false,
            version: "".to_string(),
            seed: "".to_string(),
            identity_id: "".to_string(),
            settings_active: false,
            receive_active: false,
            has_wallet: false,
            copied: false,
            confirmed_balance: HashMap::new(),
            pending_balance: HashMap::new(),
            addresses: Vec::new(),
            shimmer_url: {
                if let Json(Ok(persisted)) = storage.restore(KEY) {
                    persisted
                } else {
                    "".to_string()
                }
            }
        };
        let mut app = App {
            link: link.clone(),
            storage,
            state,
            fetcher: None,
            timeout: None,
            timeout_callback: link.callback(|_| Msg::TimeoutDone),
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
            Msg::EnterURL=> {
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
            Msg::ShowIcon=> {
                self.state.initted = true;
            }
            Msg::FetchDone(path, data)=> {
                self.state.fetching = false;
                self.parse_json_response(path, data);
                // wallet is there! load data               // reload balance
                if (path=="check" && self.state.has_wallet) || path=="faucet" {
                    self.fetch_json("state", json!({}));
                }
            }
            Msg::FetchErr(err)=> {
                warn!("{:?}",err)
            }
            Msg::Create=> {
                self.fetch_json("create", json!({
                    "url": self.state.shimmer_url
                })); 
            }
            Msg::SeedCopied=> {
                web::coopy(self.state.seed.as_str());
                self.state.seed = "".to_string();
                self.state.has_wallet = true;    
            }
            Msg::AddressCopied(addy)=> {
                web::coopy(addy.as_str());
                self.state.copied = true;
                let handle = TimeoutService::spawn(Duration::from_secs(3), self.timeout_callback.clone());
                self.timeout = Some(Box::new(handle));
            }
            Msg::TimeoutDone=> {
                self.state.copied = false;
            }
            Msg::Balance=> {
                self.fetch_json("state", json!({})); 
            }
            Msg::SettingsClicked=> {
                self.state.receive_active = false;
                self.state.settings_active = !self.state.settings_active;
            }
            Msg::ReceiveClicked=> {
                self.state.settings_active = false;
                self.state.receive_active = !self.state.receive_active;
            }
            Msg::FaucetClicked=> {
                if !self.state.fetching {
                    self.state.fetching = true;
                    self.fetch_json("faucet", json!({}));
                }
            }
            Msg::Mint=> {
                // let coin = Coin {
                //     description: self.state.value.clone(),
                //     completed: false,
                //     editing: false,
                // };
                // self.state.coins.push(coin);
            }
            Msg::Nope=> {}
        }
        self.storage.store(KEY, Json(&self.state.shimmer_url));
        info!("{:?}",self.state);
        true
    }

    fn view(&self) -> Html {
        self.view_app()
    }
}

