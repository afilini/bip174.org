#![allow(unused_imports)]
use log::*;

use bitcoin::hash_types::Txid;
use bitcoin::hashes::hex::ToHex;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::util::bip32::KeySource;
use bitcoin::util::psbt::Input;
use bitcoin::{OutPoint, PublicKey, Script, Transaction, TxIn, TxOut, SigHashType};
use std::collections::BTreeMap;
use yew::prelude::*;

use crate::app::WeakComponentLink;

use super::IOBoxMsg;

#[derive(Clone, Debug)]
pub struct InputBox {
    is_expanded: bool,
    index: usize,

    txid: Txid,
    vout: u32,
    value: Option<u64>,
    witness_utxo: Option<TxOut>,
    non_witness_utxo: Option<Transaction>,
    sighash_type: Option<SigHashType>,
    signatures: BTreeMap<PublicKey, Vec<u8>>,
    hd_keypaths: BTreeMap<PublicKey, KeySource>,
    witness_script: Option<Script>,
    redeem_script: Option<Script>,
    final_script_witness: Option<Vec<Vec<u8>>>,
    final_script_sig: Option<Script>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    pub input: Input,
    pub txin: TxIn,

    pub index: usize,

    pub weak_link: WeakComponentLink<InputBox>,
}

impl InputProps {
    fn get_value(&self) -> Option<u64> {
        let value = if let Some(utxo) = &self.input.witness_utxo {
            Some(utxo.value)
        } else if let Some(utxo) = &self.input.non_witness_utxo {
            utxo.output
                .get(self.txin.previous_output.vout as usize)
                .map(|u| u.value)
        } else {
            None
        };

        value
    }
}

impl Component for InputBox {
    type Message = IOBoxMsg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        *props.weak_link.borrow_mut() = Some(link);

        let OutPoint { txid, vout } = props.txin.previous_output;

        InputBox {
            is_expanded: false,
            index: props.index,

            txid,
            vout,
            value: props.get_value(),
            witness_utxo: props.input.witness_utxo,
            non_witness_utxo: props.input.non_witness_utxo,
            sighash_type: props.input.sighash_type,
            signatures: props.input.partial_sigs,
            hd_keypaths: props.input.hd_keypaths,
            witness_script: props.input.witness_script,
            redeem_script: props.input.redeem_script,
            final_script_witness: props.input.final_script_witness,
            final_script_sig: props.input.final_script_sig,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let OutPoint { txid, vout } = props.txin.previous_output;

        self.index = props.index;
        self.txid = txid;
        self.vout = vout;
        self.value = props.get_value();
        self.witness_utxo = props.input.witness_utxo;
        self.non_witness_utxo = props.input.non_witness_utxo;
        self.sighash_type = props.input.sighash_type;
        self.signatures = props.input.partial_sigs;
        self.hd_keypaths = props.input.hd_keypaths;
        self.witness_script = props.input.witness_script;
        self.redeem_script = props.input.redeem_script;
        self.final_script_witness = props.input.final_script_witness;
        self.final_script_sig = props.input.final_script_sig;

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            IOBoxMsg::ToggleExpand(status) => self.is_expanded = status,
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card mb-3 position-relative">
                // <span class="position-absolute top-0 start-100 translate-middle text-success fs-5"><i class="bi bi-check-circle-fill"></i></span> TODO

                <div class="card-header d-flex justify-content-between flex-wrap">
                    <div class="col-12 col-md-9 d-flex">
                        <span class="fw-light pe-1">{ format!("#{}", self.index) }</span>
                        <a class="text-break" href="#">{ format!("{}:{}", self.txid, self.vout ) }</a>
                    </div>
                    <span class="offset-1 col-11 offset-md-0 col-md-3 text-end">{ format!("{} BTC", self.value.map(|v| format!("{:.8}", (v as f32 * 1e-8))).unwrap_or("???".into())) }</span> // TODO: change unit
                </div>

                {
                    if self.is_expanded {
                        html!{}
                    } else {
                        self.emit_short_body()
                    }
                }
            </div>
        }
    }
}

struct BoxFieldGroup {
    title: &'static str,
    fields: Vec<(&'static str, Option<String>)>,
}

impl BoxFieldGroup {
    fn emit_html(&self, is_expanded: bool) -> Html {
        html! {
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ self.title }</span>

                  <div class="col-7 font-monospace">
                  { for self.fields.iter().enumerate().map(|(index, (title, opt))| {
                        if is_expanded {
                            html!{}
                        } else {
                            let class = match opt {
                                Some(_) => "",
                                None => "text-muted",
                            };
                            let separator = if index == 0 {
                                html!{}
                            } else {
                                html!{ <i class="bi bi-dot"></i> }
                            };

                            html!{
                                <span>
                                    { separator }
                                    <span class=class>{ title }</span>
                                </span>
                            }
                        }
                  }) }
                  </div>
              </div>
        }
    }
}

struct BoxFieldSingle {
    title: &'static str,
    value: Option<String>,
}

impl BoxFieldSingle {
    fn emit_html(&self, is_expanded: bool) -> Html {
        html! {
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ self.title }</span>

                  <div class="col-7 font-monospace">
                  {
                      if is_expanded {
                          html! {}
                      } else {
                          let class = match &self.value {
                              Some(_) => "",
                              None => "text-muted",
                          };

                          html! { <span class=class>{ self.value.as_ref().unwrap_or(&"default".to_string()) }</span> }
                      }
                  }
                  </div>
              </div>
        }
    }
}

struct BoxFieldProperty<P: ToString> {
    title: &'static str,
    property: P,
}

impl<P: ToString> BoxFieldProperty<P> {
    fn emit_html(&self, is_expanded: bool) -> Html {
        html! {
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ self.title }</span>

                  <div class="col-7 font-monospace">
                  {
                      if is_expanded {
                          html! {}
                      } else {
                          html! { <span>{ self.property.to_string() }</span> }
                      }
                  }
                  </div>
              </div>
        }
    }
}

struct BoxFieldMap {
    title: &'static str,
    map: BTreeMap<String, String>,
}

impl BoxFieldMap {
    fn emit_html(&self, is_expanded: bool) -> Html {
        html! {
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ self.title }</span>

                  <div class="col-7 font-monospace">
                  {
                      if is_expanded {
                          html! {}
                      } else {
                          html! { <span>{ self.map.len().to_string() }</span> }
                      }
                  }
                  </div>
              </div>
        }
    }
}

impl InputBox {
    fn emit_short_body(&self) -> Html {
        let previous_utxo = BoxFieldGroup {
            title: "Previous UTXO",
            fields: vec![
                ("Witness", self.witness_utxo.as_ref().map(|d| serialize_hex(d))),
                ("Legacy", self.non_witness_utxo.as_ref().map(|d| serialize_hex(d))),
            ],
        };
        let partial_sigs = BoxFieldMap {
            title: "Partial Signatures",
            map: self.signatures.iter().map(|(k, v)| (k.to_string(), v.to_hex())).collect(),
        };
        let hd_keypaths = BoxFieldMap {
            title: "BIP32 Key Paths",
            map: self.hd_keypaths.iter().map(|(k, (f, p))| (k.to_string(), format!("{} {}", f, p))).collect(),
        };
        let sighash_type = BoxFieldSingle {
            title: "SigHash Type",
            value: self.sighash_type.map(|s| s.to_string())
        };
        let finalized = BoxFieldProperty {
            title: "Finalized",
            property: self.final_script_sig.is_some() || self.final_script_witness.is_some(),
        };
        let spending_script = BoxFieldGroup {
            title: "Spending Script",
            fields: vec![
                ("Witness", self.witness_script.as_ref().map(|d| serialize_hex(d))),
                ("Legacy", self.redeem_script.as_ref().map(|d| serialize_hex(d))),
            ],
        };

        html!{
            <div>
                { previous_utxo.emit_html(false) }

                { partial_sigs.emit_html(false) }

                { hd_keypaths.emit_html(false) }

                { sighash_type.emit_html(false) }

                { finalized.emit_html(false) }

                { spending_script.emit_html(false) }
            </div>
        }
    }
}
