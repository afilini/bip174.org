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
            IOBoxMsg::ToggleExpand(status) => info!("toggle expand to {}", status),
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
                { self.txid }
                <br/>
                { format!("{:?}", self.vout) }
                <br/>
                { format!("{:?}", self.value) }
            </div>
        }
    }
}
