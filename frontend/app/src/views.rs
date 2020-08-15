use yew::prelude::*;
use crate::components::{grid::Grid, page::Page, create::Create};
use crate::components::icons::{logo::Logo, gear::Gear, loading::Loading, wallet::Wallet, iota::IOTA, left::Left, plus::Plus, edit::Edit};

use crate::app::{App, Msg, Coin};

const IOTA_COLOR: &str = "IOTA";

impl App {

// pending in ()
// interval to update balances (after some actions, faster interval for a bit)
// 3 3 3 9 9 9 27 27 27 81...
pub fn view_app(&self) -> Html {
    html! {
        <main class="wrapper">
            <Grid done=self.link.callback(|_| Msg::ShowLogo) />
            <div class="app">
                {self.view_sidebar()}
                {self.view_content()}
            </div>
        </main>
    }
}

pub fn view_coins(&self) -> Html {
    html! {
        <div class="coins">
            {for self.state.coins.iter().enumerate().map(|e| self.view_coin(e)) }
        </div>
    }
}

pub fn view_coin(&self, (_idx, coin): (usize, &Coin)) -> Html {
    let color = coin.color.clone();
    let balance = match self.state.confirmed_balance.get(&coin.color) {
        Some(n)=>n, None=>&0u64
    };
    let pending = match self.state.pending_balance.get(&coin.color) {
        Some(n)=>n, None=>&0u64
    };
    let is_selected = self.state.selected_color==color;
    let view_logo = || {
        if coin.color.clone()==IOTA_COLOR {
            return html!{<IOTA />}
        }
        html!{}
    };
    html! {
        <div class=if is_selected {"coin selected"} else {"coin"} 
        onclick=self.link.callback(move |_| Msg::CoinClicked(color.clone()))>
            <div class="coin-left">
                {view_logo()}
                <div class="coin-name">{&coin.name}</div>
            </div>
            <div class="coin-balance">{balance}</div>
        </div>
    }
}

pub fn view_sidebar(&self) -> Html {
    let view_plus = || {
        if self.state.synced {
            return html!{<Plus active=self.state.creating loading=false
                onclick=self.link.callback(|_| Msg::CreateClicked)
            />}
        }
        html!{}
    };
    html!{<section class="sidebar">
        <header class=if self.state.initted {"sidebar-head"} else {"sidebar-head hide"}>
            <div class="sidebar-left">
                <Logo />
                <div class="title">{"BROOD WALLET"}</div>
            </div>
            {view_plus()}
        </header>
        <div class="sidebar-body">
            {self.view_coins()}
        </div>
    </section>}
}

pub fn view_content(&self) -> Html {
    if !self.state.initted {
        return html! {}
    }
    if self.state.checking && !self.state.settings_active {
        return html!{<section class="content-center">
            <Loading size="" />
        </section>}
    }
    if self.state.shimmer_url.len()==0 {
        return self.view_url_input()
    }
    html! {
        <section class="content">
            <header class="content-header">
                {self.view_receive()}
                {self.view_info()}
            </header>
            <div class="content-body">
                {self.view_body()}
            </div>
        </section>
    }
}

pub fn view_settings(&self) -> Html {
    if !self.state.settings_active {
        return html!{}
    }
    let view_url_button_content = || {
        if !self.state.checking {
            return html!{"OK"}
        }
        return html!{<Loading size="small" />}
    };
    html!{<>
        <div>{&self.state.identity_id}</div>
        <div>{&self.state.version}</div>
        <aside class="set-url">
            <div class="settings-url-wrap">
                <button class="button settings-url-button"
                    visibility=if !self.state.changing_url {"hidden"} else {""}
                    disabled=self.state.url_input_value.len()==0
                    onclick=self.link.callback(|_| Msg::EnterChangedURL)
                >
                    {view_url_button_content()}
                </button>
                <div class="small-url">
                    <input class="settings-url-input"
                        visibility=if !self.state.changing_url {"hidden"} else {""}
                        placeholder="Shimmer URL"
                        value=&self.state.url_input_value
                        oninput=self.link.callback(|e: InputData| Msg::UpdateURL(e.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { Msg::EnterChangedURL } else { Msg::Nope }
                        })
                    />
                    <span visibility=if self.state.changing_url {"hidden"} else {""}>
                        {&self.state.shimmer_url}
                    </span>
                </div>
            </div>
            <Edit active=self.state.changing_url 
                onclick=self.link.callback(|_| Msg::PencilClicked)
            />
        </aside>
    </>}
}

pub fn view_info(&self) -> Html {
    let mut synced_text = "NOT SYNCED";
    if self.state.synced {
        synced_text = "SYNCED";
    }
    html!{
        <div class="node-info">
            <Gear active=self.state.settings_active 
                onclick=self.link.callback(|_| Msg::SettingsClicked)
            />
            <div class="synced">{synced_text}</div>
            {self.view_settings()}
        </div>
    }
}

pub fn view_receive(&self) -> Html {
    let receiver = self.state.addresses.iter().find(|&addy| addy.is_receive);
    let receive_address = match receiver {
        Some(a)=> a.address.clone(),
        None=>"".to_string(),
    };
    if receive_address.len()==0{
        return html!{<div></div>};
    }
    html!{<div class="receive-wrap">
        <div class=if self.state.receive_active {"receive show"} else {"receive"}>
            <Left onclick=self.link.callback(|_| Msg::ReceiveClicked) />
            <div class="receive-input-wrap">
                <div class="receive-label">
                    {"Receive Address:"}
                </div>
                <input readonly=true class="receive-addy" value={&receive_address} />
            </div>
            <button class=if self.state.copied {"receive-copy button copied"} else {"receive-copy button"}
                onclick=self.link.callback(move |_| Msg::AddressCopied(receive_address.clone()))>
                {if self.state.copied {"COPIED!"} else {"COPY"}}
            </button>
            <Wallet active=self.state.receive_active
                onclick=self.link.callback(|_| Msg::ReceiveClicked)
            />
        </div>
    </div>}
}

pub fn view_body(&self) -> Html {
    if !self.state.has_wallet && self.state.seed.len()==0 && self.state.synced {
        return html!{
            <div class="create">
                <p>{"Create your wallet!"}</p>
                <button class="button create-button"
                    onclick=self.link.callback(|_| Msg::Create)
                >{"CREATE"}</button>
            </div>
        }
    }
    if !self.state.has_wallet && self.state.seed.len()>0 {
        return html!{
            <div class="show-seed">
                <p>{"Copy and save your seed. It will not be shown again!"}</p>
                <pre>{&self.state.seed}</pre>
                <button class="button seed-button"
                    onclick=self.link.callback(|_| Msg::SeedCopied)
                >{"COPY SEED"}</button>
            </div>
        }
    }
    if self.state.selected_color.len()>0 {
        let balance = match self.state.confirmed_balance.get(&self.state.selected_color) {
            Some(n)=>n, None=>&0u64
        };
        let pending = match self.state.pending_balance.get(&self.state.selected_color) {
            Some(n)=>n, None=>&0u64
        };
        let coin = self.state.coins.iter().find(|&c| c.color==self.state.selected_color );
        // info!("{:?}",coin);
        return match coin {
            Some(c)=> html!{<Page
                coin={c}
                balance={balance}
                pending={pending}
                reload={self.link.callback(|_| Msg::Reload)}
            />},
            None=>html!{},
        }; 
    }
    if self.state.creating {
        let iota_balance = match self.state.confirmed_balance.get(IOTA_COLOR) {
            Some(n)=>n, None=>&0u64
        };
        return html!{<Create
            reload={self.link.callback(|_| Msg::Reload)}
            created={self.link.callback(|args:(Coin,u64)| Msg::CoinCreated(args.0,args.1))}
            iota_balance=iota_balance
        />}
    }
    html!{}
}

pub fn view_url_input(&self) -> Html {
    html! {
        <section class="content-center">
            <div class="url-input-wrap">
                <input class="url-input"
                    placeholder="Shimmer URL"
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

}
