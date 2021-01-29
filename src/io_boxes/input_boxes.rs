#![allow(unused_imports)]
use log::*;

use bitcoin::consensus::encode::serialize_hex;
use bitcoin::hash_types::Txid;
use bitcoin::hashes::hex::ToHex;
use bitcoin::util::bip32::KeySource;
use bitcoin::util::psbt::Input;
use bitcoin::{OutPoint, PublicKey, Script, SigHashType, Transaction, TxIn, TxOut};
use std::collections::BTreeMap;
use yew::prelude::*;

use crate::app::WeakComponentLink;

use super::{BoxField, BoxFieldValue};

#[derive(Clone, Debug)]
pub struct InputBox {
    is_expanded: bool,
    is_editable: bool,
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

    pub is_expanded: bool,
    pub is_editable: bool,
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
    type Message = ();
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        *props.weak_link.borrow_mut() = Some(link);

        let OutPoint { txid, vout } = props.txin.previous_output;

        InputBox {
            is_expanded: props.is_expanded,
            is_editable: props.is_editable,
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

        self.is_expanded = props.is_expanded;
        self.is_editable = props.is_editable;
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
                    self.emit_body()
                }
            </div>
        }
    }
}

impl InputBox {
    fn emit_body(&self) -> Html {
        html! {
            <div>
                <BoxField title="Previous UTXO" expanded=self.is_expanded value=BoxFieldValue::Group({
                    vec![
                        (
                            "Witness",
                            self.witness_utxo.as_ref().map(|d| serialize_hex(d)),
                        ),
                        (
                            "Legacy",
                            self.non_witness_utxo.as_ref().map(|d| serialize_hex(d)),
                        ),
                    ]
                })/>
                <BoxField title="Partial Signatures" expanded=self.is_expanded value=BoxFieldValue::Map({
                    self
                        .signatures
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_hex()))
                        .collect()
                })/>
                <BoxField title="BIP32 Key Paths" expanded=self.is_expanded value=BoxFieldValue::Map({
                    self
                        .hd_keypaths
                        .iter()
                        .map(|(k, (f, p))| (format!("{}", f), format!("{}", p)))
                        .collect()
                })/>
                <BoxField title="SigHash Type" expanded=self.is_expanded value=BoxFieldValue::Single({
                    self.sighash_type.map(|s| s.to_string())
                })/>
                <BoxField title="Finalized" expanded=self.is_expanded value=BoxFieldValue::BooleanSummary({
                    let mut map = BTreeMap::new();
                    map.insert(
                        "Script Sig",
                        self.final_script_sig.as_ref().map(|s| serialize_hex(s)),
                    );
                    map.insert(
                        "Witness",
                        self.final_script_witness.as_ref().map(|w| {
                            w.iter()
                                .map(|v| serialize_hex(v))
                                .collect::<Vec<_>>()
                                .join(" ")
                        }),
                    );
                    map
                })/>
                <BoxField title="Spending Script" expanded=self.is_expanded value=BoxFieldValue::Group({
                    vec![
                        (
                            "Witness",
                            self.witness_script.as_ref().map(|d| serialize_hex(d)),
                        ),
                        (
                            "Legacy",
                            self.redeem_script.as_ref().map(|d| serialize_hex(d)),
                        ),
                    ]
                })/>
            </div>
        }
    }
}
