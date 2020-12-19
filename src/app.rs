use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;

use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::consensus::deserialize;

pub struct App {
    link: ComponentLink<Self>,
    psbt: Option<Result<PartiallySignedTransaction, String>>,
}

pub enum Msg {
    PsbtChanged(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            psbt: None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PsbtChanged(s) => {
                info!("{}", s);

                let decoded = base64::decode(&s).map_err(|e| e.to_string());
                let psbt = decoded.and_then(|bytes| deserialize(&bytes).map_err(|e| e.to_string()));

                self.psbt = Some(psbt);

                info!("{:?}", self.psbt);
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1>{ "bip174.org" }</h1>

                { self.render_psbt_textarea() }
            </div>
        }
    }
}

impl App {
    fn render_psbt_textarea(&self) -> Html {
        let mut classes = Classes::new().extend("form-control");
        let error = if let Some(Err(e)) = &self.psbt {
            classes.push("is-invalid");

            html! {
                <div class="invalid-feedback">
                    { e }
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <div class="form-floating">
                <textarea class={ classes } id="psbtInput" oninput=self.link.callback(move |e: InputData| Msg::PsbtChanged(e.value))></textarea>
                <label for="psbtInput">{ "PSBT" }</label>
                { error }
            </div>
        }
    }
}
