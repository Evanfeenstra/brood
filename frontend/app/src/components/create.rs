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
use crate::components::icons::{plus::Plus};
use crate::utils::valid;

pub struct Create {
    link: ComponentLink<Self>,
    state: State,
    props: Props,
    pub fetcher: Option<FetchTask>,
}

struct State {
    name: String,
    symbol: String,
    amount: String,
    creating: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub reload: Callback<()>,
    pub created: Callback<(Coin,u64)>,
    pub iota_balance: u64,
}

pub enum Msg {
    UpdateName(String),
    UpdateSymbol(String),
    UpdateAmount(String),
    FetchDone(&'static str, String),
    FetchErr(anyhow::Error, String),
    CreateClicked,
    Nope,
}

impl Component for Create {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State{
            name: "".to_string(),
            symbol: "".to_string(),
            amount: "".to_string(),
            creating: false,
        };
        Create {
            link,
            state,
            props,
            fetcher: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(val) => {
                if val.len() < 42 {
                    self.state.name = val;
                }
            }
            Msg::UpdateSymbol(val) => {
                if val.len() < 18 {
                    self.state.symbol = val;
                }
            }
            Msg::UpdateAmount(val) => {
                if valid::amount_input(&val, self.props.iota_balance) {
                    self.state.amount = val;
                }
            }
            Msg::FetchDone(path, data)=> {
                self.parse_json_response(path, data);
            }
            Msg::FetchErr(err, _path)=> {
                log::warn!("{:?}",err);
                self.state.creating = false;
                self.state.amount = "".to_string();
                self.state.symbol = "".to_string();
                self.state.name = "".to_string();
            }
            Msg::CreateClicked=> {
                // info!("{:?}", "create clicked");
                if self.state.creating {
                    return false
                }
                let amt = match self.state.amount.parse::<i64>() {
                    Ok(n)=> n,
                    Err(_e)=> 0,
                };
                self.state.creating = true;
                self.fetch_json("coin", json!({
                    "name": self.state.name,
                    "symbol": self.state.symbol,
                    "amount": amt,
                }));
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
            <div class="create">
                <div class="create-name">{"NEW COIN"}</div>
                {self.view_inputs()}
                {self.view_button()}
            </div>
        }
    }
}

// views
impl Create {

pub fn view_button(&self) -> Html {
    html!{<div class="create-button-wrap">
        <button class="create-button button"
            onclick=self.link.callback(|_| Msg::CreateClicked)
            disabled=self.state.name.len()==0 || self.state.symbol.len()==0 || self.state.amount.len()==0>
            <Plus loading=self.state.creating active=false
                onclick=self.link.callback(|_| Msg::Nope)
            />
            {"CREATE"}
        </button>
    </div>}
}

pub fn view_inputs(&self) -> Html {
    html! {<div class="create-inputs">
        <input class="create-input"
            placeholder="Name"
            value=&self.state.name
            oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
        />
        <input class="create-input"
            placeholder="Symbol"
            value=&self.state.symbol
            oninput=self.link.callback(|e: InputData| Msg::UpdateSymbol(e.value))
        />
        <input class="create-input"
            placeholder="Amount"
            value=&self.state.amount
            oninput=self.link.callback(|e: InputData| Msg::UpdateAmount(e.value))
        />
    </div>}
}

}

// fetcher stuff
static URL: &'static str = "http://localhost:3579/";

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRes {
    coin: Coin,
}

impl Create {

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
pub fn parse_json_response(&mut self, path:&'static str, r:String){
    match path {
        "coin"=>{
            let amt = match self.state.amount.parse::<u64>() {
                Ok(n)=> n,
                Err(_e)=> 0,
            };
            let json: Result<CreateRes,Error> = from_str(r.as_str());
            json.map(|data| {
                self.props.created.emit((data.coin, amt));
            }).ok();
            self.state.creating = false;
        }
        &_=>()
    }
}

}
