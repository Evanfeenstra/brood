
use yew::prelude::*;
use crate::components::{grid::Grid};
use crate::components::icons::{logo::Logo, gear::Gear, loading::Loading, wallet::Wallet, faucet::Faucet};

use crate::app::{App, Msg};

impl App {

pub fn view_app(&self) -> Html {
    html! {
        <main class="wrapper">
            <Grid done=self.link.callback(|_| Msg::ShowIcon) />
            <div class="app">
                {self.view_sidebar()}
                {self.view_content()}
            </div>
        </main>
    }
}

pub fn view_coins(&self) -> Html {
    html! {
        <div>{"."}</div>
    }
}

pub fn view_sidebar(&self) -> Html {
    html!{<section class="sidebar">
        <header class=if self.state.initted {"sidebar-head"} else {"sidebar-head hide"}>
            <Logo />
            <div class="title">{"brood wallet"}</div>
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
    if self.state.fetching {
        return html!{<section class="content-center">
            <Loading big={true} />
        </section>}
    }
    if self.state.shimmer_url.len()==0 {
        return self.view_url_input()
    }
    let mut synced_text = "NOT SYNCED";
    if self.state.synced {
        synced_text = "SYNCED";
    }
    let view_settings = || {
        if self.state.settings_active {
            return html!{<>
                <div>{&self.state.version}</div>
                <div>{&self.state.identity_id}</div>
            </>}
        }
        html!{}
    };
    html! {
        <section class="content">
            <header class="content-header">
                {self.view_receive()}
                <div class="node-info">
                    {view_settings()}
                    <div>{synced_text}</div>
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

pub fn view_receive(&self) -> Html {
    let mut receive_address="".to_string();
    for addy in self.state.addresses.iter() {
        match addy.is_receive {
            true=> receive_address=addy.address.clone(),
            false=>()
        }
    };
    return html!{<div class="receive-wrap">
        <div class=if self.state.receive_active {"receive show"} else {"receive"}>
            <div class="receive-input-wrap">
                <div class="receive-label">
                    {"Receive Address:"}
                </div>
                <input readonly=true class="receive-addy" value={receive_address.clone()} />
            </div>
            <button class=if self.state.copied {"receive-copy button copied"} else {"receive-copy button"}
                onclick=self.link.callback(move |_| Msg::AddressCopied(receive_address.clone()))>
                {if self.state.copied {"COPIED!"} else {"COPY"}}
            </button>
            <Faucet active=self.state.fetching
                onclick=self.link.callback(|_| Msg::FaucetClicked)
            />
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
                >{"CREATE NOW"}</button>
            </div>
        }
    }
    if !self.state.has_wallet {
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
    html!{
        
    }
}

pub fn view_url_input(&self) -> Html {
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

}
