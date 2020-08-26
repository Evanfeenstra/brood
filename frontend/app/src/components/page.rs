use yew::prelude::*;
use serde_json::{value::Value, error::Error, from_str, json};
use yew::format::Json;
use yew::services::{
    fetch::{FetchService, Request, Response, FetchTask},
};
use serde_derive::{Deserialize, Serialize};
use yew::format::{Text};
use anyhow::{anyhow};
use crate::app::{Coin};
use crate::components::icons::{faucet::Faucet, send::Send, edit::Edit};
use crate::utils::valid;

const IOTA_COLOR: &str = "IOTA";
const EMPTY_NAME: &str = "••••••";
const EMPTY_SYMBOL: &str = "cI";

pub struct Page {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
    pub fetcher: Option<FetchTask>,
}

struct State {
    synced: bool,
    addy: String,
    amount: String,
    fetching_faucet: bool,
    sending: bool,
    name: String,
    color: String,
    symbol: String,
    balance: u64,
    pending: u64,
    meta_key_down: bool,
    editing: bool,
    edit_name: String,
    edit_symbol: String,
    registering: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub synced: bool,
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
    InputKeyEvent(String,String,String),
    PencilClicked,
    UpdateName(String),
    UpdateSymbol(String),
    EditCoin,
    Nope,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let initial_props = props.clone();
        let state = State{
            synced: initial_props.synced,
            name: initial_props.coin.name,
            color: initial_props.coin.color,
            symbol: initial_props.coin.symbol,
            balance: initial_props.balance,
            pending: initial_props.pending,
            addy: "".to_string(),
            amount: "".to_string(),
            fetching_faucet: false,
            sending: false,
            meta_key_down: false,
            editing: false,
            edit_name: "".to_string(),
            edit_symbol: "".to_string(),
            registering: false,
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
                if valid::address(&val) {
                    self.state.addy = val;
                }
            }
            Msg::UpdateAmount(val) => {
                if valid::amount_input(&val, self.state.balance) {
                    self.state.amount = val;
                }
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
                    "color": self.state.color,
                }));
            }
            Msg::FetchDone(path, data)=> {
                if path=="faucet"  {
                    self.state.fetching_faucet = false;
                }
                if path=="send" {
                    self.state.sending = false;
                    self.state.amount = "".to_string();
                    self.state.addy = "".to_string();
                }
                if path=="clipboard" {
                    let json: Result<ClipboardRes,Error> = from_str(data.as_str());
                    json.map(|data| {
                        if data.meta == "addy" && data.cmd=="paste" {
                            if valid::address(&data.text) {
                                self.state.addy = data.text;
                            }
                        }
                    }).ok();
                }
                if path=="register" {
                    self.state.editing = false;
                    self.state.registering = false;
                }
                self.props.reload.emit(());
            }
            Msg::FetchErr(err, path)=> {
                log::warn!("{:?}",err);
                match path.as_str() {
                    "faucet"=> self.state.fetching_faucet = false,
                    "send"=> self.state.sending = false,
                    &_=>()
                }
            }
            Msg::FaucetClicked=> {
                if self.state.fetching_faucet {
                    return false
                }
                self.state.fetching_faucet = true;
                self.fetch_json("faucet", json!({}));
            }
            Msg::InputKeyEvent(direction,key,field)=> {
                if key == "Meta" || key == "Control" {
                    self.state.meta_key_down = direction=="down";
                }
                if self.state.meta_key_down {
                    if key=="c" || key=="x" {
                        self.fetch_json("clipboard", json!({
                            "cmd": "copy",
                            "text": self.state.addy,
                            "meta": field,
                        }));
                    }
                    if key=="v" {
                        self.fetch_json("clipboard", json!({
                            "cmd": "paste",
                            "text": self.state.addy,
                            "meta": field,
                        }));
                    }
                }
            }
            Msg::PencilClicked=>{
                if self.state.editing {
                    self.state.edit_name = "".to_string();
                    self.state.edit_symbol = "".to_string();
                }
                self.state.editing = !self.state.editing;
            }
            Msg::UpdateName(val) => {
                if val.len() < 42 {
                    self.state.edit_name = val;
                }
            }
            Msg::UpdateSymbol(val)=> {
                if val.len() < 18 {
                    self.state.edit_symbol = val;
                }
            }
            Msg::EditCoin=> {
                self.state.registering = true;
                self.fetch_json("register", json!({
                    "color": self.state.color,
                    "name": self.state.edit_name,
                    "symbol": self.state.edit_symbol,
                }));
            }
            Msg::Nope=> {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.state.color != props.coin.color {
            self.state.editing = false;
            self.state.edit_name = "".to_string();
            self.state.edit_symbol = "".to_string();
        }
        self.state.name = props.coin.name;
        self.state.color = props.coin.color;
        self.state.symbol = props.coin.symbol;
        self.state.balance = props.balance;
        self.state.pending = props.pending;
        self.state.synced = props.synced;
        true
    }

    fn view(&self) -> Html {
        if self.state.editing {
            return self.view_edit();
        };
        html! {
            <div class="page" color=self.state.color>
                <div class="page-name-wrap">
                    <div class="page-name">
                        {&self.state.name}
                    </div>
                    <div class="page-edit"
                        visibility=if self.state.name!=EMPTY_NAME {"hidden"} else {""}>
                        <Edit active=self.state.editing 
                            onclick=self.link.callback(|_| Msg::PencilClicked)
                        />
                    </div>
                </div>
                <div class="page-color"
                    display=if self.state.color==IOTA_COLOR {"none"} else {""}>
                    <div class="page-symbol" visibility=if self.state.symbol==EMPTY_SYMBOL {"hidden"} else {""}>
                        {&self.state.symbol}
                    </div>
                    <b>{"Color: "}</b>
                    <span>{&self.state.color}</span>
                </div>
                <div class="page-balance">
                    {"Balance: "}
                    {&self.state.balance}
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
    if self.state.pending==0 {
        return html!{}
    }
    html!{<div class="page-pending">
        {"Pending:  "}
        {&self.state.pending}
    </div>}
}

pub fn view_send(&self) -> Html {
    html! {<div class="send">
        <input class="send-input"
            placeholder="Send to Address"
            value=&self.state.addy
            oninput=self.link.callback(|e: InputData| Msg::UpdateAddy(e.value))
            onkeyup=self.link.callback(|e: KeyboardEvent| Msg::InputKeyEvent("up".to_string(),e.key(),"addy".to_string()))
            onkeydown=self.link.callback(|e: KeyboardEvent| Msg::InputKeyEvent("down".to_string(),e.key(),"addy".to_string()))
            oncopy=self.link.callback(|e: Event| {e.prevent_default(); Msg::Nope})
            oncut=self.link.callback(|e: Event| {e.prevent_default(); Msg::Nope})
            onpaste=self.link.callback(|e: Event| {e.prevent_default(); Msg::Nope})
        />
        <input class="amount-input"
            type="number"
            placeholder="Amount"
            value=&self.state.amount
            oninput=self.link.callback(|e: InputData| Msg::UpdateAmount(e.value))
        />
        <button class="button send-button"
            active={self.state.sending}
            disabled=self.state.addy.len()!=44 || self.state.amount.len()==0
            onclick=self.link.callback(|_| Msg::EnterAddy)
        >
            <Send active={self.state.sending} />
            <span>{"SEND"}</span>
        </button>
    </div>}
}

pub fn view_faucet(&self) -> Html {
    if self.state.color!=IOTA_COLOR {
        return html!{<div></div>}
    }
    html!{<div class="faucet-wrap">
        <button class="button faucet-button"
            disabled={!self.state.synced}
            active={self.state.fetching_faucet}
            onclick=self.link.callback(|_| Msg::FaucetClicked)>
            <Faucet active={self.state.fetching_faucet} />
            <span>{"Faucet"}</span>
        </button>
    </div>}
}

pub fn view_edit(&self) -> Html {
    html!{<div class="page" color=self.state.color>
        <div class="edit-coin-inputs">
            <input class="edit-coin-input edit-name"
                placeholder="Name"
                value=&self.state.edit_name
                oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
            />
            <input class="edit-coin-input edit-symbol"
                placeholder="Symbol"
                value=&self.state.edit_symbol
                oninput=self.link.callback(|e: InputData| Msg::UpdateSymbol(e.value))
            />
            <button class="button save-button"
                disabled=self.state.registering || self.state.edit_name.len()==0 || self.state.edit_symbol.len()==0
                onclick=self.link.callback(|_| Msg::EditCoin)
            >
                <span>{"SAVE"}</span>
            </button>
            <div class="page-edit"
                visibility=if self.state.name!=EMPTY_NAME {"hidden"} else {""}>
                <Edit active=self.state.editing 
                    onclick=self.link.callback(|_| Msg::PencilClicked)
                />
            </div>
        </div>
    </div>}
}

}

// fetcher stuff
static URL: &'static str = "http://localhost:3888/";

#[derive(Serialize, Deserialize, Debug)]
pub struct FaucetRes {
    success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SendRes {
    success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClipboardRes {
    cmd: String,
    text: String,
    meta: String,
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

}
