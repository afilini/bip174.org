use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

#[allow(unused_imports)]
use log::*;
use yew::prelude::*;

use bitcoin::util::bip32;
use bitcoin::util::psbt::{self, PartiallySignedTransaction};
use bitcoin::{Address, Network, Script, SigHashType, Transaction, TxIn, TxOut};

use crate::bootstrap::*;
use crate::fields::*;
use crate::history::*;
use crate::navbar::*;

const EXAMPLE_PSBTS: [(&str, &str); 3] = [
    ("One P2PKH input, outputs are empty", "cHNidP8BAHUCAAAAASaBcTce3/KF6Tet7qSze3gADAVmy7OtZGQXE8pCFxv2AAAAAAD+////AtPf9QUAAAAAGXapFNDFmQPFusKGh2DpD9UhpGZap2UgiKwA4fUFAAAAABepFDVF5uM7gyxHBQ8k0+65PJwDlIvHh7MuEwAAAQD9pQEBAAAAAAECiaPHHqtNIOA3G7ukzGmPopXJRjr6Ljl/hTPMti+VZ+UBAAAAFxYAFL4Y0VKpsBIDna89p95PUzSe7LmF/////4b4qkOnHf8USIk6UwpyN+9rRgi7st0tAXHmOuxqSJC0AQAAABcWABT+Pp7xp0XpdNkCxDVZQ6vLNL1TU/////8CAMLrCwAAAAAZdqkUhc/xCX/Z4Ai7NK9wnGIZeziXikiIrHL++E4sAAAAF6kUM5cluiHv1irHU6m80GfWx6ajnQWHAkcwRAIgJxK+IuAnDzlPVoMR3HyppolwuAJf3TskAinwf4pfOiQCIAGLONfc0xTnNMkna9b7QPZzMlvEuqFEyADS8vAtsnZcASED0uFWdJQbrUqZY3LLh+GFbTZSYG2YVi/jnF6efkE/IQUCSDBFAiEA0SuFLYXc2WHS9fSrZgZU327tzHlMDDPOXMMJ/7X85Y0CIGczio4OFyXBl/saiK9Z9R5E5CVbIBZ8hoQDHAXR8lkqASECI7cr7vCWXRC+B3jv7NYfysb3mk6haTkzgHNEZPhPKrMAAAAAAAAA"),
    ("One P2SH-P2WSH input of a 2-of-2, with metadata", "cHNidP8BAFUCAAAAASeaIyOl37UfxF8iD6WLD8E+HjNCeSqF1+Ns1jM7XLw5AAAAAAD/////AaBa6gsAAAAAGXapFP/pwAYQl8w7Y28ssEYPpPxCfStFiKwAAAAAAAEBIJVe6gsAAAAAF6kUY0UgD2jRieGtwN8cTRbqjxTA2+uHIgIDsTQcy6doO2r08SOM1ul+cWfVafrEfx5I1HVBhENVvUZGMEMCIAQktY7/qqaU4VWepck7v9SokGQiQFXN8HC2dxRpRC0HAh9cjrD+plFtYLisszrWTt5g6Hhb+zqpS5m9+GFR25qaAQEEIgAgdx/RitRZZm3Unz1WTj28QvTIR3TjYK2haBao7UiNVoEBBUdSIQOxNBzLp2g7avTxI4zW6X5xZ9Vp+sR/HkjUdUGEQ1W9RiED3lXR4drIBeP4pYwfv5uUwC89uq/hJ/78pJlfJvggg71SriIGA7E0HMunaDtq9PEjjNbpfnFn1Wn6xH8eSNR1QYRDVb1GELSmumcAAACAAAAAgAQAAIAiBgPeVdHh2sgF4/iljB+/m5TALz26r+En/vykmV8m+CCDvRC0prpnAAAAgAAAAIAFAACAAAA="),
    ("Revault Unvault TX", "cHNidP8BAIkCAAAAAV+HumeWIAtm1c9hvTgUme25aogn3EvF1+vV7KYKKKdYAAAAAAD9////AkANAwAAAAAAIgAgXA0s+qynDjinXOmpJ/Qhuj87xEB7YcLEVdz7OX5B+l8wdQAAAAAAACIAIKj/nBsC9abIRvrVxbaHRVSZNtMZjsOSosgybAbmDAtwAAAAAAABASuIlAMAAAAAACIAIKI1Ly2kCXvsF5kWmgyAGmH2th23XwgbIDHRo7sHndheAQMEAQAAAAEFR1IhAtk/sjHYB5gv7nUSr0k25UlmeCn+7ztrilD5aKBYhOZ/IQI+TfqYOB5AvGLZO2C3OWNepPtB2MXltlovJy9aNEUezFKuIgYCPk36mDgeQLxi2TtgtzljXqT7QdjF5bZaLycvWjRFHswIeMYQoQoAAAAiBgLZP7Ix2AeYL+51Eq9JNuVJZngp/u87a4pQ+WigWITmfwgbQV1zCgAAAAAAAA=="),
];

pub trait ParentMessage<T, X = ()> {
    fn build_message(data: T, tag: Option<X>) -> Self;
}

#[derive(Clone)]
pub struct WeakComponentLink<COMP: Component>(Rc<RefCell<Option<ComponentLink<COMP>>>>);

pub struct App {
    link: ComponentLink<Self>,
    network: Network,

    psbt: WeakComponentLink<Psbt>,
}

#[derive(Debug)]
pub enum AppMsg {
    SetNetwork(Network),
    SetPsbt(&'static str),

    Undo,
    Redo,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            network: Network::Testnet,
            psbt: WeakComponentLink(Rc::new(RefCell::new(None))),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let send_psbt_message = |m| {
            if let Some(link) = self.psbt.0.borrow().deref() {
                link.send_message(m);
            }
        };

        match msg {
            AppMsg::SetNetwork(network) => self.network = network,
            AppMsg::SetPsbt(psbt) => send_psbt_message(PsbtMessage::ChangePsbt(
                PartiallySignedTransaction::from_str(psbt).ok(),
            )),
            AppMsg::Undo => send_psbt_message(PsbtMessage::Undo),
            AppMsg::Redo => send_psbt_message(PsbtMessage::Redo),
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <ContainerFluid>
                <Navbar network=self.network parent=self.link.clone() />

                <Container>
                    <div class="d-flex justify-content-between align-items-center">
                        <h2 class="my-3">{ "Bitcoin PSBT Explorer" }</h2>
                        <div class="dropdown">
                            <button class="btn btn-outline-secondary dropdown-toggle" type="button" id="examplesDropdown" data-bs-toggle="dropdown" aria-expanded="false">
                                { "Examples "}
                            </button>
                                <ul class="dropdown-menu dropdown-menu-end" aria-labelledby="examplesDropdown">
                                    {
                                        for EXAMPLE_PSBTS.iter().map(|(label, psbt)| html! {
                                                <li><a class="dropdown-item" onclick=self.link.callback(move |_| AppMsg::SetPsbt(psbt))>{ label }</a></li>
                                        })
                                    }
                                </ul>
                        </div>
                    </div>

                    <Psbt network=self.network self_link=WeakComponentLink(Rc::clone(&self.psbt.0)) />
                </Container>
            </ContainerFluid>
        }
    }
}

#[derive(Clone)]
pub struct Psbt {
    link: ComponentLink<Self>,
    props: PsbtProps,

    psbt: Option<PartiallySignedTransaction>,
    history: History,
}

#[derive(Clone, Properties)]
pub struct PsbtProps {
    pub network: Network,
    pub self_link: WeakComponentLink<Psbt>,
}

#[derive(Clone, Debug)]
pub enum PsbtMessage {
    ChangePsbt(Option<PartiallySignedTransaction>),
    ChangeInput(usize, PsbtInputMsg),
    ChangeOutput(usize, PsbtOutputMsg),

    Undo,
    Redo,

    None,
}

impl ParentMessage<Option<PartiallySignedTransaction>> for PsbtMessage {
    fn build_message(data: Option<PartiallySignedTransaction>, _tag: Option<()>) -> Self {
        PsbtMessage::ChangePsbt(data)
    }
}

impl PsbtMessage {
    pub fn apply_to(self, psbt: &mut Option<PartiallySignedTransaction>) -> PsbtMessage {
        match self {
            PsbtMessage::ChangePsbt(new_psbt) => {
                let old = psbt.take();
                *psbt = new_psbt;

                PsbtMessage::ChangePsbt(old)
            }
            PsbtMessage::ChangeInput(index, msg) => psbt
                .as_mut()
                .map(|psbt| PsbtMessage::ChangeInput(index, msg.apply_to(&mut psbt.inputs[index])))
                .unwrap_or(PsbtMessage::None),
            PsbtMessage::ChangeOutput(index, msg) => psbt
                .as_mut()
                .map(|psbt| {
                    PsbtMessage::ChangeOutput(index, msg.apply_to(&mut psbt.outputs[index]))
                })
                .unwrap_or(PsbtMessage::None),
            _ => PsbtMessage::None,
        }
    }
}

impl Component for Psbt {
    type Message = PsbtMessage;
    type Properties = PsbtProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        *props.self_link.0.borrow_mut() = Some(link.clone());

        Psbt {
            link,
            psbt: None,
            history: Default::default(),
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        *self.props.self_link.0.borrow_mut() = Some(self.link.clone());

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PsbtMessage::Undo => self.history.undo(&mut self.psbt),
            PsbtMessage::Redo => self.history.redo(&mut self.psbt),
            msg => {
                let opposite = msg.apply_to(&mut self.psbt);
                self.history.add(opposite);

                true
            }
        }
    }

    fn view(&self) -> Html {
        type SingleFieldPsbt = SingleField<Option<PartiallySignedTransaction>, Psbt, (), 1>;

        html! {
            <Container class="p-0">
                <Row>
                    <Column xs=12>
                       <SingleFieldPsbt value=self.psbt.clone() label="PSBT" parent=self.link.clone() />
                    </Column>

                    {
                        self.psbt.as_ref().map(|_| html! {
                            <div class="d-flex flex-wrap">
                                // Inputs
                                <Column xs=12 md=6 class="order-first">
                                    <h2 class="my-3">{ "Inputs" }</h2>
                                    {
                                        self.psbt.as_ref().map(|psbt| html! { for psbt.inputs.iter().zip(psbt.global.unsigned_tx.input.iter()).enumerate().map(|(index, (psbt_input, input))| html!{ <PsbtInput index=index input=input.clone() psbt_input=psbt_input.clone() network=self.props.network parent=self.link.clone() /> }) }).unwrap_or_default()
                                    }
                                </Column>

                                // Left/bottom arrow
                                <div class="align-self-center text-center col-1 d-none d-md-block" style="padding-top: 4.4em;">
                                    <h1><i class="bi bi-chevron-right"></i></h1>
                                </div>
                                <div class="mt-3 align-self-center text-center col-12 d-block d-md-none">
                                    <h1 class="m-0"><i class="bi bi-chevron-down"></i></h1>
                                </div>

                                // Outputs
                                <Column xs=12 md=5 class="order-last">
                                    <h2 class="my-3">{ "Outputs" }</h2>
                                    {
                                        self.psbt.as_ref().map(|psbt | html! { for psbt.outputs.iter().zip(psbt.global.unsigned_tx.output.iter()).enumerate().map(|(index, (psbt_output, output))| html!{ <PsbtOutput index=index output=output.clone() psbt_output=psbt_output.clone() network=self.props.network parent=self.link.clone() /> }) }).unwrap_or_default()
                                    }
                                </Column>
                            </div>
                        }).unwrap_or_default()
                    }
                </Row>
            </Container>
        }
    }
}

pub trait Field<const N: usize>: Clone + Sized + std::fmt::Debug {
    type DeserializeError: std::fmt::Debug;

    fn serialize(&self) -> [String; N];
    fn deserialize(s: [&str; N]) -> Result<Self, Self::DeserializeError>;
}

impl<T: Field<N>, const N: usize> Field<N> for Option<T> {
    type DeserializeError = <T as Field<N>>::DeserializeError;

    fn serialize(&self) -> [String; N] {
        match self {
            None => {
                let v = vec![String::new(); N];
                v.try_into().unwrap()
            }
            Some(s) => s.serialize(),
        }
    }
    fn deserialize(s: [&str; N]) -> Result<Self, Self::DeserializeError> {
        if s.iter().any(|s| s.is_empty()) {
            Ok(None)
        } else {
            Ok(Some(T::deserialize(s)?))
        }
    }
}

macro_rules! impl_hex_serialize_field {
    ($type:ty) => {
        impl Field<1> for $type {
            type DeserializeError = ParseError;

            fn deserialize(s: [&str; 1]) -> Result<Self, Self::DeserializeError> {
                use bitcoin::consensus::encode::deserialize;
                use bitcoin::hashes::hex::FromHex;

                let data = Vec::<u8>::from_hex(s[0])?;
                Ok(deserialize(&data)?)
            }

            fn serialize(&self) -> [String; 1] {
                use bitcoin::consensus::encode::serialize;
                use bitcoin::hashes::hex::ToHex;

                [serialize(&self).to_hex()]
            }
        }
    };
}
impl_hex_serialize_field!(TxOut);
impl_hex_serialize_field!(Transaction);
impl_hex_serialize_field!(Vec<Vec<u8>>);

impl Field<1> for bitcoin::PublicKey {
    type DeserializeError = ParseError;

    fn deserialize(s: [&str; 1]) -> Result<Self, Self::DeserializeError> {
        use bitcoin::hashes::hex::FromHex;
        let data = Vec::<u8>::from_hex(s[0])?;
        Ok(bitcoin::PublicKey::from_slice(&data)?)
    }

    fn serialize(&self) -> [String; 1] {
        use bitcoin::hashes::hex::ToHex;
        [self.to_bytes().to_hex()]
    }
}

impl Field<1> for Script {
    type DeserializeError = ParseError;

    fn deserialize(s: [&str; 1]) -> Result<Self, Self::DeserializeError> {
        use bitcoin::hashes::hex::FromHex;
        Ok(Script::from_hex(s[0])?)
    }

    fn serialize(&self) -> [String; 1] {
        use bitcoin::hashes::hex::ToHex;
        [self.to_hex()]
    }
}

impl Field<1> for Vec<u8> {
    type DeserializeError = ParseError;

    fn deserialize(s: [&str; 1]) -> Result<Self, Self::DeserializeError> {
        use bitcoin::hashes::hex::FromHex;

        Ok(Vec::<u8>::from_hex(s[0])?)
    }

    fn serialize(&self) -> [String; 1] {
        use bitcoin::hashes::hex::ToHex;

        [self.to_hex()]
    }
}

impl Field<1> for PartiallySignedTransaction {
    type DeserializeError = psbt::PsbtParseError;

    fn deserialize(s: [&str; 1]) -> Result<Self, Self::DeserializeError> {
        PartiallySignedTransaction::from_str(s[0])
    }

    fn serialize(&self) -> [String; 1] {
        [self.to_string()]
    }
}

impl Field<2> for bitcoin::util::bip32::KeySource {
    type DeserializeError = ParseError;

    fn deserialize(s: [&str; 2]) -> Result<Self, Self::DeserializeError> {
        use bitcoin::hashes::hex::FromHex;

        let fingerprint = FromHex::from_hex(s[0])?;
        let path = FromStr::from_str(s[1])?;

        Ok((fingerprint, path))
    }

    fn serialize(&self) -> [String; 2] {
        use bitcoin::hashes::hex::ToHex;

        [self.0.to_hex(), self.1.to_string()]
    }
}

macro_rules! declare_ty_wrapper {
    ($name:ident, $wrap:ty $(, with_ord $tt:tt)?) => {
        #[derive(Debug, Clone, PartialEq, Eq $(, PartialOrd, Ord $tt )? )]
        pub struct $name($wrap);

        impl std::ops::Deref for $name {
            type Target = $wrap;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl From<$wrap> for $name {
            fn from(t: $wrap) -> Self {
                $name(t)
            }
        }
        impl From<$name> for $wrap {
            fn from(t: $name) -> Self {
                t.0
            }
        }

        impl Field<1> for $name {
            type DeserializeError = ParseError;

            fn serialize(&self) -> [String; 1] {
                self.0.serialize()
            }
            fn deserialize(s: [&str; 1]) -> Result<Self, Self::DeserializeError> {
                Ok(<$wrap>::deserialize(s)?.into())
            }
        }
    };
}

declare_ty_wrapper!(WitnessUtxo, Option<TxOut>);
declare_ty_wrapper!(NonWitnessUtxo, Option<Transaction>);
declare_ty_wrapper!(PublicKeyWrapper, bitcoin::PublicKey, with_ord,);
declare_ty_wrapper!(BytesWrapper, Vec<u8>);
declare_ty_wrapper!(RedeemScript, Option<Script>);
declare_ty_wrapper!(WitnessScript, Option<Script>);
declare_ty_wrapper!(FinalScript, Option<Script>);
declare_ty_wrapper!(FinalWitness, Option<Vec<Vec<u8>>>);

fn build_row(item: Html) -> Html {
    html! {
        <div class="card-body py-2 d-flex">
            <Column xs=12>
                { item }
            </Column>
        </div>
    }
}

#[derive(Clone, Properties)]
pub struct PsbtInputProps {
    index: usize,
    psbt_input: psbt::Input,
    input: TxIn,

    network: Network,
    parent: ComponentLink<Psbt>,
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum PsbtInputMsg {
    ChangeSigHash(Option<SigHashType>),
    ChangeWitnessUtxo(WitnessUtxo),
    ChangeNonWitnessUtxo(NonWitnessUtxo),
    ChangeRedeemScript(RedeemScript),
    ChangeWitnessScript(WitnessScript),
    ChangeFinalScript(FinalScript),
    ChangeFinalWitness(FinalWitness),
    ChangePartialSigs(MapUpdate<PublicKeyWrapper, BytesWrapper>),
    ChangeBIP32Derivation(MapUpdate<PublicKeyWrapper, bip32::KeySource>),
}
macro_rules! impl_parent_message {
    ($enum:ident, $variant:ident, $type:ty) => {
        impl ParentMessage<$type> for $enum {
            fn build_message(t: $type, _tag: Option<()>) -> Self {
                $enum::$variant(t)
            }
        }
    };
}
impl_parent_message!(PsbtInputMsg, ChangeSigHash, Option<SigHashType>);
impl_parent_message!(PsbtInputMsg, ChangeWitnessUtxo, WitnessUtxo);
impl_parent_message!(PsbtInputMsg, ChangeNonWitnessUtxo, NonWitnessUtxo);
impl_parent_message!(PsbtInputMsg, ChangeRedeemScript, RedeemScript);
impl_parent_message!(PsbtInputMsg, ChangeWitnessScript, WitnessScript);
impl_parent_message!(PsbtInputMsg, ChangeFinalScript, FinalScript);
impl_parent_message!(PsbtInputMsg, ChangeFinalWitness, FinalWitness);
impl_parent_message!(PsbtInputMsg, ChangePartialSigs, MapUpdate<PublicKeyWrapper, BytesWrapper>);
impl_parent_message!(PsbtInputMsg, ChangeBIP32Derivation, MapUpdate<PublicKeyWrapper, bip32::KeySource>);

macro_rules! set_and_return {
    ($field:expr, $val:expr) => {{
        let old = $field.clone();
        $field = $val;
        old.into()
    }};
}

impl PsbtInputMsg {
    pub fn apply_to(self, psbt_input: &mut psbt::Input) -> PsbtInputMsg {
        match self {
            PsbtInputMsg::ChangeSigHash(sighash) => {
                PsbtInputMsg::ChangeSigHash(set_and_return!(psbt_input.sighash_type, sighash))
            }
            PsbtInputMsg::ChangeWitnessUtxo(witness_utxo) => PsbtInputMsg::ChangeWitnessUtxo(
                set_and_return!(psbt_input.witness_utxo, witness_utxo.0),
            ),
            PsbtInputMsg::ChangeNonWitnessUtxo(non_witness_utxo) => {
                PsbtInputMsg::ChangeNonWitnessUtxo(set_and_return!(
                    psbt_input.non_witness_utxo,
                    non_witness_utxo.0
                ))
            }
            PsbtInputMsg::ChangeRedeemScript(redeem_script) => PsbtInputMsg::ChangeRedeemScript(
                set_and_return!(psbt_input.redeem_script, redeem_script.0),
            ),
            PsbtInputMsg::ChangeWitnessScript(witness_script) => PsbtInputMsg::ChangeWitnessScript(
                set_and_return!(psbt_input.witness_script, witness_script.0),
            ),
            PsbtInputMsg::ChangeFinalScript(final_script_sig) => PsbtInputMsg::ChangeFinalScript(
                set_and_return!(psbt_input.final_script_sig, final_script_sig.0),
            ),
            PsbtInputMsg::ChangeFinalWitness(final_script_witness) => {
                PsbtInputMsg::ChangeFinalWitness(set_and_return!(
                    psbt_input.final_script_witness,
                    final_script_witness.0
                ))
            }
            PsbtInputMsg::ChangePartialSigs(c) => {
                let prev = match c {
                    MapUpdate::Set(k, v) => (
                        k.clone(),
                        psbt_input.partial_sigs.insert(k.into(), v.into()),
                    ),
                    MapUpdate::Remove(k) => (k.clone(), psbt_input.partial_sigs.remove(&k)),
                };
                match prev {
                    (k, Some(v)) => PsbtInputMsg::ChangePartialSigs(MapUpdate::Set(k, v.into())),
                    (k, None) => PsbtInputMsg::ChangePartialSigs(MapUpdate::Remove(k)),
                }
            }
            PsbtInputMsg::ChangeBIP32Derivation(c) => {
                let prev = match c {
                    MapUpdate::Set(k, v) => {
                        (k.clone(), psbt_input.bip32_derivation.insert(k.into(), v))
                    }
                    MapUpdate::Remove(k) => (k.clone(), psbt_input.bip32_derivation.remove(&k)),
                };
                match prev {
                    (k, Some(v)) => PsbtInputMsg::ChangeBIP32Derivation(MapUpdate::Set(k, v)),
                    (k, None) => PsbtInputMsg::ChangeBIP32Derivation(MapUpdate::Remove(k)),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PsbtInput {
    link: ComponentLink<Self>,
    props: PsbtInputProps,
}

impl Component for PsbtInput {
    type Message = PsbtInputMsg;
    type Properties = PsbtInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PsbtInput { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props
            .parent
            .send_message(PsbtMessage::ChangeInput(self.props.index, msg));

        false
    }

    fn view(&self) -> Html {
        type SingleFieldWitnessUtxo = SingleField<WitnessUtxo, PsbtInput, (), 1>;
        type SingleFieldNonWitnessUtxo = SingleField<NonWitnessUtxo, PsbtInput, (), 1>;
        type SingleFieldRedeemScript = SingleField<RedeemScript, PsbtInput, (), 1>;
        type SingleFieldWitnessScript = SingleField<WitnessScript, PsbtInput, (), 1>;
        type SingleFieldFinalScript = SingleField<FinalScript, PsbtInput, (), 1>;
        type SingleFieldFinalWitness = SingleField<FinalWitness, PsbtInput, (), 1>;
        type SelectFieldSigHash = SelectField<SigHashType, PsbtInput, ()>;
        type MapFieldPartialSigs = MapField<PublicKeyWrapper, BytesWrapper, PsbtInput, (), 1, 1>;
        type MapFieldBIP32Derivation =
            MapField<PublicKeyWrapper, bip32::KeySource, PsbtInput, (), 1, 2>;

        let partial_sigs = self
            .props
            .psbt_input
            .partial_sigs
            .iter()
            .map(|(k, v)| ((*k).into(), v.clone().into()))
            .collect::<BTreeMap<_, _>>();
        let bip32_derivation = self
            .props
            .psbt_input
            .bip32_derivation
            .iter()
            .map(|(k, v)| ((*k).into(), v.clone()))
            .collect::<BTreeMap<_, _>>();

        html! {
            <div class="card mb-3 pb-2 position-relative">
                <div class="card-header mb-2 d-flex flex-wrap">
                    <span class="col-1 fw-light">{ format!("#{}", self.props.index) }</span>
                    <span class="col-11">{ self.props.input.previous_output.to_string() }</span>
                    // <span class="offset-1 col-11 offset-md-0 col-md-3 text-end">{ "??? BTC" }</span>
                </div>

                { build_row(html! { <SingleFieldWitnessUtxo label="Witness UTXO" value=WitnessUtxo(self.props.psbt_input.witness_utxo.clone()) parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldNonWitnessUtxo label="Non Witness UTXO" value=NonWitnessUtxo(self.props.psbt_input.non_witness_utxo.clone()) parent=self.link.clone() /> }) }
                { build_row(html! { <MapFieldPartialSigs label="Partial Signatures" key_label="Public Key" value_label="Signature" map=partial_sigs parent=self.link.clone() /> }) }
                { build_row(html! { <MapFieldBIP32Derivation label="BIP32 Derivation" key_label="Public Key" value_label=["Fingerprint", "Path"] map=bip32_derivation parent=self.link.clone() /> }) }
                { build_row(html! { <SelectFieldSigHash label="Sighash Type".to_string() allow_empty=true selected=self.props.psbt_input.sighash_type values=vec![SigHashType::All, SigHashType::None, SigHashType::Single, SigHashType::AllPlusAnyoneCanPay, SigHashType::NonePlusAnyoneCanPay, SigHashType::SinglePlusAnyoneCanPay] parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldFinalScript label="Final Script Sig" value=FinalScript(self.props.psbt_input.final_script_sig.clone()) parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldFinalWitness label="Final Script Witness" value=FinalWitness(self.props.psbt_input.final_script_witness.clone()) parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldRedeemScript label="Redeem Script" value=RedeemScript(self.props.psbt_input.redeem_script.clone()) parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldWitnessScript label="Witness Script" value=WitnessScript(self.props.psbt_input.witness_script.clone()) parent=self.link.clone() /> }) }
            </div>
        }
    }
}

#[derive(Clone, Properties)]
pub struct PsbtOutputProps {
    index: usize,
    psbt_output: psbt::Output,
    output: TxOut,

    network: Network,
    parent: ComponentLink<Psbt>,
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum PsbtOutputMsg {
    ChangeRedeemScript(RedeemScript),
    ChangeWitnessScript(WitnessScript),
    ChangeBIP32Derivation(MapUpdate<PublicKeyWrapper, bip32::KeySource>),
}
impl_parent_message!(PsbtOutputMsg, ChangeRedeemScript, RedeemScript);
impl_parent_message!(PsbtOutputMsg, ChangeWitnessScript, WitnessScript);
impl_parent_message!(PsbtOutputMsg, ChangeBIP32Derivation, MapUpdate<PublicKeyWrapper, bip32::KeySource>);

impl PsbtOutputMsg {
    fn apply_to(self, psbt_output: &mut psbt::Output) -> PsbtOutputMsg {
        match self {
            PsbtOutputMsg::ChangeRedeemScript(redeem_script) => PsbtOutputMsg::ChangeRedeemScript(
                set_and_return!(psbt_output.redeem_script, redeem_script.0),
            ),
            PsbtOutputMsg::ChangeWitnessScript(witness_script) => {
                PsbtOutputMsg::ChangeWitnessScript(set_and_return!(
                    psbt_output.witness_script,
                    witness_script.0
                ))
            }
            PsbtOutputMsg::ChangeBIP32Derivation(c) => {
                let prev = match c {
                    MapUpdate::Set(k, v) => {
                        (k.clone(), psbt_output.bip32_derivation.insert(k.into(), v))
                    }
                    MapUpdate::Remove(k) => (k.clone(), psbt_output.bip32_derivation.remove(&k)),
                };
                match prev {
                    (k, Some(v)) => PsbtOutputMsg::ChangeBIP32Derivation(MapUpdate::Set(k, v)),
                    (k, None) => PsbtOutputMsg::ChangeBIP32Derivation(MapUpdate::Remove(k)),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PsbtOutput {
    link: ComponentLink<Self>,
    props: PsbtOutputProps,
}

impl Component for PsbtOutput {
    type Message = PsbtOutputMsg;
    type Properties = PsbtOutputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PsbtOutput { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props
            .parent
            .send_message(PsbtMessage::ChangeOutput(self.props.index, msg));
        false
    }

    fn view(&self) -> Html {
        type SingleFieldRedeemScript = SingleField<RedeemScript, PsbtOutput, (), 1>;
        type SingleFieldWitnessScript = SingleField<WitnessScript, PsbtOutput, (), 1>;
        type MapFieldBIP32Derivation =
            MapField<PublicKeyWrapper, bip32::KeySource, PsbtOutput, (), 1, 2>;

        let bip32_derivation = self
            .props
            .psbt_output
            .bip32_derivation
            .iter()
            .map(|(k, v)| ((*k).into(), v.clone()))
            .collect::<BTreeMap<_, _>>();

        html! {
            <div class="card mb-3 pb-2 position-relative">
                <div class="card-header mb-2 d-flex flex-wrap">
                    <span class="col-1 fw-light">{ format!("#{}", self.props.index) }</span>
                    <span class="col-11">{ Address::from_script(&self.props.output.script_pubkey, self.props.network).map(|a| a.to_string()).unwrap_or_else(|| self.props.output.script_pubkey.to_string()) }</span>
                    // <span class="offset-1 col-11 offset-md-0 col-md-3 text-end">{ "??? BTC" }</span>
                </div>

                { build_row(html! { <MapFieldBIP32Derivation label="BIP32 Derivation" key_label="Public Key" value_label=["Fingerprint", "Path"] map=bip32_derivation parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldRedeemScript label="Redeem Script" value=RedeemScript(self.props.psbt_output.redeem_script.clone()) parent=self.link.clone() /> }) }
                { build_row(html! { <SingleFieldWitnessScript label="Witness Script" value=WitnessScript(self.props.psbt_output.witness_script.clone()) parent=self.link.clone() /> }) }
            </div>
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Hex(bitcoin::hashes::hex::Error),
    Encode(bitcoin::consensus::encode::Error),
    Key(bitcoin::util::key::Error),
    BIP32(bitcoin::util::bip32::Error),
}
impl From<bitcoin::hashes::hex::Error> for ParseError {
    fn from(e: bitcoin::hashes::hex::Error) -> Self {
        ParseError::Hex(e)
    }
}
impl From<bitcoin::consensus::encode::Error> for ParseError {
    fn from(e: bitcoin::consensus::encode::Error) -> Self {
        ParseError::Encode(e)
    }
}
impl From<bitcoin::util::key::Error> for ParseError {
    fn from(e: bitcoin::util::key::Error) -> Self {
        ParseError::Key(e)
    }
}
impl From<bitcoin::util::bip32::Error> for ParseError {
    fn from(e: bitcoin::util::bip32::Error) -> Self {
        ParseError::BIP32(e)
    }
}
