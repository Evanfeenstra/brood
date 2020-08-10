use yew::prelude::*;
use log::{info, warn};
use serde_json::{value::Value, error::Error, from_str, json};
use yew::format::Json;
use yew::services::{
    fetch::{FetchService, Request, Response, FetchTask},
};
use serde_derive::{Deserialize, Serialize};
use yew::format::{Text};
use anyhow::{anyhow};
use crate::app::{Coin};
use crate::components::icons::{faucet::Faucet, send::Send};
pub struct Page {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
    pub fetcher: Option<FetchTask>,
}

struct State {
    addy: String,
    amount: String,
    fetching_faucet: bool,
    sending: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub coin: Coin,
    pub balance: u64,
    pub pending: u64,
    pub reload: Callback<()>,
}

pub enum Msg {
    UpdateAddy(String),
    EnterAddy,
    UpdateAmount(String),
    FetchDone(&'static str, String),
    FetchErr(anyhow::Error, String),
    FaucetClicked,
    Nope,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State{
            addy: "".to_string(),
            amount: "".to_string(),
            fetching_faucet: false,
            sending: false,
        };
        Page {
            link,
            state,
            props,
            fetcher: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateAddy(val) => {
                self.state.addy = val;
            }
            Msg::UpdateAmount(val) => {
                self.state.amount = val;
            }
            Msg::EnterAddy=> {
                if self.state.sending {
                    return false;
                }
                let amt = match self.state.amount.parse::<i64>() {
                    Ok(n)=> n,
                    Err(_e)=> 0,
                };
                self.state.sending = true;
                self.fetch_json("send", json!({
                    "address": self.state.addy,
                    "amount": amt,
                    "color": self.props.coin.color,
                }));
            }
            Msg::FetchDone(path, data)=> {
                self.parse_json_response(path, data);
                if path=="faucet"  {
                    self.state.fetching_faucet = false;
                    self.props.reload.emit(());
                }
                if path=="send" {
                    self.state.sending = false;
                    self.props.reload.emit(());
                    self.state.amount = "".to_string();
                    self.state.addy = "".to_string();
                }
            }
            Msg::FetchErr(err, path)=> {
                warn!("{:?}",err);
                self.handle_error(path)
            }
            Msg::FaucetClicked=> {
                if self.state.fetching_faucet {
                    return false
                }
                self.state.fetching_faucet = true;
                self.fetch_json("faucet", json!({}));
            }
            Msg::Nope=> {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="page">
                <div class="page-name">{&self.props.coin.name}</div>
                <div class="page-balance">
                    {"Balance:  "}
                    {&self.props.balance}
                </div>
                {self.view_pending()}
                {self.view_send()}
                {self.view_faucet()}
            </div>
        }
    }
}

// views
impl Page {

pub fn view_pending(&self) -> Html {
    if self.props.pending==0 {
        return html!{}
    }
    html!{<div class="pending">
        {"Pending:  "}
        {&self.props.pending}
    </div>}
}

pub fn view_send(&self) -> Html {
    html! {<div class="send">
        <input class="send-input"
            placeholder="Send to Address"
            value=&self.state.addy
            oninput=self.link.callback(|e: InputData| Msg::UpdateAddy(e.value))
        />
        <input class="amount-input"
            type="number"
            placeholder="Amount"
            value=&self.state.amount
            oninput=self.link.callback(|e: InputData| Msg::UpdateAmount(e.value))
        />
        <button class="button send-button"
            disabled=self.state.addy.len()==0 || self.state.amount.len()==0
            onclick=self.link.callback(|_| Msg::EnterAddy)
        >
            <Send active={self.state.sending} />
            <span>{"SEND"}</span>
        </button>
    </div>}
}

pub fn view_faucet(&self) -> Html {
    if self.props.coin.color!="IOTA" {
        return html!{<div></div>}
    }
    return html!{<div class="faucet-wrap">
        <button class="button faucet-button"
            onclick=self.link.callback(|_| Msg::FaucetClicked)>
            <Faucet active={self.state.fetching_faucet} />
            <span>{"Faucet"}</span>
        </button>
    </div>}
}

}

// fetcher stuff
static URL: &'static str = "http://localhost:3579/";

#[derive(Serialize, Deserialize, Debug)]
pub struct FaucetRes {
    success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SendRes {
    success: bool,
}

impl Page {

pub fn fetch_json(&mut self, path:&'static str, body: Value) {
    let callback = self.link.callback(
        move |response: Response<Text>| {
            let (meta, data) = response.into_parts();
            if meta.status.is_success() {
                match data {
                    Ok(d)=> Msg::FetchDone(path, d),
                    Err(e)=> Msg::FetchErr(e, path.to_string()),
                }
            } else {
                info!("error: {:?}",meta.status);
                Msg::FetchErr(anyhow!("cant fetch"), path.to_string())
            }
        },
    );
    match Request::post(URL.to_string()+&path).body(Json(&body)) {
        Ok(req) => {
            let res = FetchService::fetch(req, callback);
            self.fetcher = Some(res.unwrap());
        },
        Err(e) => { Msg::FetchErr(anyhow::Error::new(e), path.to_string()); }
    };
}
pub fn parse_json_response(&mut self, path:&'static str, r:String){
    match path {
        "faucet"=>{
            let json: Result<FaucetRes,Error> = from_str(r.as_str());
            info!("faucet successful: {:?}", json);
        }
        "send"=>{
            let json: Result<SendRes,Error> = from_str(r.as_str());
            info!("faucet successful: {:?}", json);
        }
        &_=>()
    }
}
pub fn handle_error(&mut self, path:String) {
    match path.as_str() {
        "faucet"=> self.state.fetching_faucet = false,
        "send"=> self.state.sending = false,
        &_=>()
    }
}

}
