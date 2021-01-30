use crate::navbar::NetworkSettings;
use bitcoin::consensus::encode::serialize_hex;
use bitcoin::util::bip32::KeySource;
use bitcoin::util::psbt::Output;
use bitcoin::{Address, PublicKey, Script, TxOut};
use std::collections::BTreeMap;
use yew::prelude::*;

use crate::io_boxes::{BoxField, BoxFieldValue};

pub struct OutputBox {
    is_expanded: bool,
    is_editable: bool,
    index: usize,

    script: Script,
    value: u64,
    hd_keypaths: BTreeMap<PublicKey, KeySource>,
    witness_script: Option<Script>,
    redeem_script: Option<Script>,
    network: NetworkSettings,
}

#[derive(Properties, Clone, PartialEq)]
pub struct OutputProps {
    pub is_expanded: bool,
    pub is_editable: bool,
    pub index: usize,

    pub output: Output,
    pub txout: TxOut,
    pub network: NetworkSettings,
}

impl Component for OutputBox {
    type Message = ();
    type Properties = OutputProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        OutputBox {
            is_expanded: props.is_expanded,
            is_editable: props.is_editable,
            index: props.index,
            script: props.txout.script_pubkey,
            value: props.txout.value,
            hd_keypaths: props.output.hd_keypaths,
            witness_script: props.output.witness_script,
            redeem_script: props.output.redeem_script,
            network: props.network,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.is_expanded = props.is_expanded;
        self.is_editable = props.is_editable;
        self.index = props.index;
        self.script = props.txout.script_pubkey;
        self.value = props.txout.value;
        self.hd_keypaths = props.output.hd_keypaths;
        self.witness_script = props.output.witness_script;
        self.redeem_script = props.output.redeem_script;
        self.network = props.network;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let address = match Address::from_script(&self.script, *self.network) {
            Some(a) => format!("{:?}", a),
            None => format!("{:?}", self.script),
        };

        html! {
            <div class="card mb-3 position-relative">
                // <span class="position-absolute top-0 start-100 translate-middle text-success fs-5"><i class="bi bi-check-circle-fill"></i></span> TODO

                <div class="card-header d-flex justify-content-between flex-wrap">
                    <div class="col-12 col-md-9 d-flex">
                        <span class="fw-light pe-1">{ format!("#{}", self.index) }</span>
                        <a class="text-break" href="#">{ format!("{}", address) }</a>
                    </div>
                    <span class="offset-1 col-11 offset-md-0 col-md-3 text-end">{ format!("{:.8} BTC", (self.value as f32 * 1e-8)) }</span> // TODO: change unit
                </div>

                {
                    self.emit_body()
                }
            </div>
        }
    }
}

impl OutputBox {
    fn emit_body(&self) -> Html {
        html! {
            <div>
                <BoxField title="BIP32 Key Paths" expanded=self.is_expanded value=BoxFieldValue::Map({
                    self
                        .hd_keypaths
                        .iter()
                        .map(|(k, (f, p))| (format!("{}", f), format!("{}", p)))
                        .collect()
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
