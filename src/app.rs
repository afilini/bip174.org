#![allow(unused_imports)]
use log::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

use strum::IntoEnumIterator;
use yew::prelude::*;

use bitcoin::consensus::deserialize;
use bitcoin::network::constants::Network;
use bitcoin::util::psbt::{Input, Output, PartiallySignedTransaction};
use bitcoin::{TxIn, TxOut};

use crate::io_boxes::{InputBox, OutputBox, IOBoxMsg};
use crate::navbar::Navbar;

#[derive(Clone, Debug)]
pub struct WeakComponentLink<C: Component>(Rc<RefCell<Option<ComponentLink<C>>>>);

impl<C: Component> Deref for WeakComponentLink<C> {
    type Target = Rc<RefCell<Option<ComponentLink<C>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C: Component> PartialEq for WeakComponentLink<C> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<C: Component> Default for WeakComponentLink<C> {
    fn default() -> Self {
        WeakComponentLink(Rc::new(RefCell::new(None)))
    }
}

pub struct App {
    link: ComponentLink<Self>,
    psbt: Option<Result<PartiallySignedTransaction, String>>,
    network: Network,

    inputs: Vec<WeakComponentLink<InputBox>>,
    outputs: Vec<WeakComponentLink<OutputBox>>,
}

pub enum Msg {
    PsbtChanged(String),

    ExpandBoxes(bool),
    EditMode(bool),
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

            inputs: vec![],
            outputs: vec![],
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

                if let Some(Ok(psbt)) = &self.psbt {
                    self.inputs = psbt.inputs.iter().map(|_| WeakComponentLink::default()).collect();
                    self.outputs = psbt.outputs.iter().map(|_| WeakComponentLink::default()).collect();
                }

                info!("{:?}", self.psbt);
            },
            Msg::ExpandBoxes(status) => {
                info!("expand_boxes={}, {:?}", status, self.inputs);

                for link in &self.inputs {
                    if let Some(link) = link.borrow().deref() {
                        link.send_message(IOBoxMsg::ToggleExpand(status));
                    }
                }
            },
            Msg::EditMode(status) => {
                info!("edit_mode={}, {:?}", status, self.inputs);
            },
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Navbar network=self.network, app_link=self.link.clone()/>

                <div class="container pb-3">
                    <h2 class="my-3">{ "Bitcoin PSBT Explorer" }</h2>

                    { self.render_psbt_textarea() }

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
                <div class="d-flex flex-wrap">
                    <div class="col-12 col-md-6 order-first">
                        <h2 class="my-3">{ "Inputs" }</h2>

                        { for psbt.inputs.iter().zip(psbt.global.unsigned_tx.input.iter()).enumerate().map(|(index, (i, txin))| self.render_input_box(i, txin, index) ) }
                    </div>

                    <div class="align-self-center text-center col-1 d-none d-md-block" style="padding-top: 4.4em;">
                        <h1><i class="bi bi-chevron-right"></i></h1>
                    </div>
                    <div class="mt-3 align-self-center text-center col-12 d-block d-md-none">
                        <h1 class="m-0"><i class="bi bi-chevron-down"></i></h1>
                    </div>

                    <div class="col-12 col-md-5 order-last">
                        <h2 class="my-3 text-md-end">{ "Outputs" }</h2>

                        { for psbt.outputs.iter().zip(psbt.global.unsigned_tx.output.iter()).map(|(o, txout)| self.render_output_box(o, txout) ) }
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn render_input_box(&self, input: &Input, txin: &TxIn, index: usize) -> Html {
        html! {
            <InputBox: input=input, txin=txin, weak_link=&self.inputs[index]/>
        }
    }

    fn render_output_box(&self, output: &Output, txout: &TxOut) -> Html {
        html! {
            <OutputBox: output=output, txout=txout, network=self.network/>
        }
    }

    fn render_psbt_textarea(&self) -> Html {
        let (error, invalid_class) = if let Some(Err(e)) = &self.psbt {
            let error_container = html! {
                <div class="invalid-feedback">
                    { e }
                </div>
            };

            (error_container, "is-invalid")
        } else {
            Default::default()
        };

        html! {
            <div class="form-floating">
                <textarea id="psbtInput" class=("form-control", invalid_class) oninput=self.link.callback(move |e: InputData| Msg::PsbtChanged(e.value))></textarea>
                <label for="psbtInput">{ "PSBT" }</label>
                { error }
            </div>
        }
    }
}
