use yew::prelude::*;
use log::{info, warn};
use serde_json::{value::Value, error::Error, from_str};
use yew::format::Json;
use yew::services::{
    fetch::{FetchService, Request, Response, FetchTask},
};
use serde_derive::{Deserialize, Serialize};
use yew::format::{Text};
use anyhow::{anyhow};
use crate::app::{Coin};

pub struct Page {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
    pub fetcher: Option<FetchTask>,
}

struct State {
    addy: String,
    fetching_faucet: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub coin: Coin,
    pub balance: u64,
    // pub color: String,
}

pub enum Msg {
    UpdateAddy(String),
    EnterAddy,
    FetchDone(&'static str, String),
    FetchErr(anyhow::Error),
    Nope,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State{
            addy: "".to_string(),
            fetching_faucet: false,
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
            Msg::EnterAddy=> {
                 
            }
            Msg::FetchDone(path, data)=> {
                self.state.fetching_faucet = false;
                self.parse_json_response(path, data);
            }
            Msg::FetchErr(err)=> {
                warn!("{:?}",err)
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
                {self.view_send()}
                {self.view_faucet()}
            </div>
        }
    }
}

// views
impl Page {

pub fn view_send(&self) -> Html {
    html! {<div class="send">
        <input class="send-input"
            placeholder="Send to Address"
            value=&self.state.addy
            oninput=self.link.callback(|e: InputData| Msg::UpdateAddy(e.value))
            onkeypress=self.link.callback(|e: KeyboardEvent| {
                if e.key() == "Enter" { Msg::EnterAddy } else { Msg::Nope }
            })
        />
        <button class="button send-button"
            disabled=self.state.addy.len()==0
            onclick=self.link.callback(|_| Msg::EnterAddy)
        >
            {"SEND"}
        </button>
    </div>}
}

pub fn view_faucet(&self) -> Html {
    if self.props.coin.color!="IOTA" {
        return html!{}
    }
    return html!{<div class="faucet-wrap">
        {"hi"}
    </div>}
}

}

// fetcher stuff
static URL: &'static str = "http://localhost:3579/";

#[derive(Serialize, Deserialize, Debug)]
pub struct FaucetRes {
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
                    Err(e)=> Msg::FetchErr(e),
                }
            } else {
                info!("error: {:?}",meta.status);
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
        "faucet"=>{
            let json: Result<FaucetRes,Error> = from_str(r.as_str());
            info!("faucet successful: {:?}", json);
        }
        &_=>()
    }
}

}
