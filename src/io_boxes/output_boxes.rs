use crate::navbar::NetworkSettings;
use bitcoin::util::bip32::KeySource;
use bitcoin::util::psbt::Output;
use bitcoin::{Address, Network, PublicKey, Script, TxOut};
use std::collections::BTreeMap;
use yew::prelude::*;

pub struct OutputBox {
    pub script: Script,
    pub value: u64,
    pub hd_keypaths: BTreeMap<PublicKey, KeySource>,
    pub witness_script: Option<Script>,
    pub redeem_script: Option<Script>,
    pub network: NetworkSettings,
}

#[derive(Properties, Clone, PartialEq)]
pub struct OutputProps {
    pub output: Output,
    pub txout: TxOut,
    pub network: NetworkSettings,
}

impl Component for OutputBox {
    type Message = ();
    type Properties = OutputProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        OutputBox {
            script: props.txout.script_pubkey,
            value: props.txout.value,
            hd_keypaths: props.output.hd_keypaths,
            witness_script: props.output.witness_script,
            redeem_script: props.output.redeem_script,
            network: props.network,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
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
        html! {
            <div class="card">
            {
                if let Some(a) = Address::from_script(&self.script, *self.network) {
                  format!("{:?}", a)
                } else {
                  format!("{:?}", self.script)
                }
            }
            </div>
        }
    }
}
