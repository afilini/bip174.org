use bitcoin::util::bip32::KeySource;
use bitcoin::util::psbt::Output;
use bitcoin::{Network, PublicKey, Script, TxOut};
use std::collections::BTreeMap;
use yew::prelude::*;

pub struct OutputBox {
    pub address: Script,
    pub value: u64,
    pub hd_keypaths: BTreeMap<PublicKey, KeySource>,
    pub witness_script: Option<Script>,
    pub redeem_script: Option<Script>,
    pub network: Network,
}

#[derive(Properties, Clone, PartialEq)]
pub struct OutputProps {
    pub output: Output,
    pub txout: TxOut,
    pub network: Network,
}

impl Component for OutputBox {
    type Message = ();
    type Properties = OutputProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        OutputBox {
            address: props.txout.script_pubkey,
            value: props.txout.value,
            hd_keypaths: props.output.hd_keypaths,
            witness_script: props.output.witness_script,
            redeem_script: props.output.redeem_script,
            network: props.network,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
                { format!("{:?}", self.address) }
            </div>
        }
    }
}
