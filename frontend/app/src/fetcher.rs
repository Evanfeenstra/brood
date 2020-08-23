use serde_derive::{Deserialize, Serialize};
use serde_json::{
    value::Value,
    error::Error,
    from_str,
    json,
};
use yew::format::Json;
use yew::services::{
    fetch::{FetchService, Request, Response},
};
use yew::format::{Text};
use anyhow::{anyhow};
use std::collections::HashMap;

use crate::app::{App, Msg, Address, Coin};

static URL: &'static str = "http://localhost:3579/";

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
#[derive(Deserialize, Debug)]
pub struct StateRes {
    confirmed_balance: HashMap<String, u64>,
    pending_balance: HashMap<String, u64>,
    coins: Vec<Coin>,
    addresses: Vec<Address>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FaucetRes {
    success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClipboardRes {
    cmd: String,
    text: String,
    meta: String,
}

impl App {
    pub fn fetch_json(&mut self, path:&'static str, body: Value) {
        let callback = self.link.callback(
            move |response: Response<Text>| {
                let (meta, data) = response.into_parts();
                if meta.status.is_success() {
                    match data {
                        Ok(d)=> Msg::FetchDone(path, d),
                        Err(e)=> Msg::FetchErr(e),
                    }
                } else {
                    // log::info!("{:?}",meta.status);
                    Msg::FetchErr(anyhow!("cant fetch"))
                }
            },
        );
        match Request::post(URL.to_string()+&path).body(Json(&body)) {
            Ok(req) => {
                let res = FetchService::fetch(req, callback);
                self.fetcher = Some(res.unwrap());
            },
            Err(e) => { Msg::FetchErr(anyhow::Error::new(e)); }
        };
    }
    pub fn parse_json_response(&mut self, path:&'static str, r:String){
        match path {
            "check"=> {
                let json: Result<CheckRes,Error> = from_str(r.as_str());
                json.map(|data| {
                    self.state.synced = data.synced;
                    self.state.version = data.version;
                    self.state.identity_id = data.identity_id;
                    self.state.has_wallet = data.has_wallet;
                }).ok();
                self.state.changing_url = false;
            }
            "create"=>{
                let json: Result<CreateRes,Error> = from_str(r.as_str());
                json.map(|data| {
                    self.state.seed = data.seed;
                }).ok();
            }
            "state"=>{
                let json: Result<StateRes,Error> = from_str(r.as_str());
                // info!("STATE RES:{:?}",json);
                json.map(|data| {
                    self.state.confirmed_balance = data.confirmed_balance;
                    self.state.pending_balance = data.pending_balance;
                    self.state.coins = data.coins;
                    self.state.addresses = data.addresses;
                }).ok();
            }
            "clipboard"=>{
                let json: Result<ClipboardRes,Error> = from_str(r.as_str());
                json.map(|data| {
                    if data.meta == "url_input_value" && data.cmd=="paste" {
                        self.state.url_input_value = data.text;
                    }
                    if data.meta=="seed" && data.cmd=="copy" {
                        self.fetch_json("state", json!({}));
                    }
                }).ok();
            }
            &_=>()
        }
    }
}

