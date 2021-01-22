#![allow(unused_imports)]
use log::*;
use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::io_boxes::{InputBox, OutputBox};
use bitcoin::consensus::deserialize;
use bitcoin::network::constants::Network;
use bitcoin::util::psbt::{Input, Output, PartiallySignedTransaction};
use bitcoin::{TxIn, TxOut};

pub struct App {
    link: ComponentLink<Self>,
    psbt: Option<Result<PartiallySignedTransaction, String>>,
    network: Network,
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
            // TODO
            network: Network::Bitcoin,
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
                <div>
                    { self.render_input_output_boxes() }
                </div>
            </div>
        }
    }
}

impl App {
    fn render_input_output_boxes(&self) -> Html {
        if let Some(Ok(psbt)) = &self.psbt {
            html! {
                <div>
                    { for psbt.inputs.iter().zip(psbt.global.unsigned_tx.input.iter()).map(|(i, txin)| self.render_input_box(i, txin) ) }
                    { for psbt.outputs.iter().zip(psbt.global.unsigned_tx.output.iter()).map(|(o, txout)| self.render_output_box(o, txout) ) }
                </div>
            }
        } else {
            html! {}
        }
    }

    fn render_input_box(&self, input: &Input, txin: &TxIn) -> Html {
        html! {
            <InputBox: input=input, txin=txin/>
        }
    }

    fn render_output_box(&self, output: &Output, txout: &TxOut) -> Html {
        html! {
            <OutputBox: output=output, txout=txout, network=self.network/>
        }
    }

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
