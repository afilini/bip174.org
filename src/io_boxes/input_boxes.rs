#![allow(unused_imports)]
use log::*;

use bitcoin::hash_types::Txid;
use bitcoin::util::bip32::KeySource;
use bitcoin::util::psbt::Input;
use bitcoin::{OutPoint, PublicKey, Script, Transaction, TxIn, TxOut};
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

impl Component for InputBox {
    type Message = IOBoxMsg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = if let Some(utxo) = &props.input.witness_utxo {
            Some(utxo.value)
        } else if let Some(utxo) = &props.input.non_witness_utxo {
            utxo.output
                .get(props.txin.previous_output.vout as usize)
                .map(|u| u.value)
        } else {
            None
        };

        let OutPoint { txid, vout } = props.txin.previous_output;

        *props.weak_link.borrow_mut() = Some(link);

        InputBox {
            is_expanded: false,
            index: props.index,

            txid,
            vout,
            value,
            witness_utxo: props.input.witness_utxo,
            non_witness_utxo: props.input.non_witness_utxo,
            signatures: props.input.partial_sigs,
            hd_keypaths: props.input.hd_keypaths,
            witness_script: props.input.witness_script,
            redeem_script: props.input.redeem_script,
            final_script_witness: props.input.final_script_witness,
            final_script_sig: props.input.final_script_sig,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ "Previous UTXO" }</span>
                  <div class="col-7 font-monospace">
                        <span class="text-muted">{ "Witness" }</span>
                        <i class="bi bi-dot"></i>
                        <span>{ "Legacy" }</span>
                  </div>
              </div>
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ "Partial Signatures" }</span>
                  <span class="col-7 font-monospace">{ "2" }</span>
              </div>
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ "BIP32 Key Paths" }</span>
                  <span class="col-7 font-monospace">{ "3" }</span>
              </div>
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ "SigHash Type" }</span>
                  <span class="col-7 font-monospace text-muted">{ "default" }</span>
              </div>
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ "Finalized" }</span>
                  <span class="col-7 font-monospace">{ "false" }</span>
              </div>
              <div class="card-body py-2 d-flex">
                  <span class="col-5 fw-bold">{ "Spending Script" }</span>
                  <div class="col-7 font-monospace">
                        <span class="text-muted">{ "Witness" }</span>
                        <i class="bi bi-dot"></i>
                        <span>{ "Legacy" }</span>
                  </div>
              </div>
            </div>
        }
    }
}
