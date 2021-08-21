use std::collections::BTreeMap;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::rc::Rc;

#[allow(unused_imports)]
use log::*;

use yew::prelude::*;
use yew::virtual_dom::VComp;

use crate::app::{Field, ParentMessage};
use crate::bootstrap::*;

pub trait BuildComponent<P: Component, const N: usize>: Sized {
    fn build_component<X: 'static + Clone + PartialEq>(
        &self,
        is_map: bool,
        tag: Option<X>,
        label: Option<[String; N]>,
        parent: ComponentLink<P>,
    ) -> VComp
    where
        <P as Component>::Message: ParentMessage<Self, X>;
}

impl<P, F, const N: usize> BuildComponent<P, N> for F
where
    F: 'static + Field<N> + PartialEq,
    P: Component + Clone,
{
    fn build_component<X: 'static + Clone + PartialEq>(
        &self,
        is_map: bool,
        tag: Option<X>,
        label: Option<[String; N]>,
        parent: ComponentLink<P>,
    ) -> VComp
    where
        <P as Component>::Message: ParentMessage<F, X>,
    {
        VComp::new::<SingleField<F, P, X, N>>(
            SingleFieldProps {
                value: self.clone(),
                size: None,
                is_scrollable: false,
                is_map,
                tag,
                label,
                parent,
            },
            NodeRef::default(),
            None,
        )
    }
}

#[derive(Clone)]
pub struct SingleFieldProps<T, P, X, const N: usize>
where
    T: Field<N> + PartialEq,
    P: Component + Clone,
    X: Clone + PartialEq,
{
    pub value: T,
    pub parent: ComponentLink<P>,
    pub tag: Option<X>,
    pub label: Option<[String; N]>,
    pub size: Option<[usize; N]>,
    pub is_map: bool,
    pub is_scrollable: bool,
}

pub struct SingleFieldPropsBuilder<T, P: Component, X, const N: usize> {
    value: Option<T>,
    parent: Option<ComponentLink<P>>,
    tag: Option<X>,
    label: Option<[String; N]>,
    size: Option<[usize; N]>,
    is_map: bool,
    is_scrollable: bool,
}

pub trait SingleOrArrayLabels<const N: usize> {
    fn into_array(self) -> [String; N];
}

impl<S: AsRef<str>, const N: usize> SingleOrArrayLabels<N> for [S; N] {
    fn into_array(self) -> [String; N] {
        self.iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
impl SingleOrArrayLabels<1> for &'_ str {
    fn into_array(self) -> [String; 1] {
        [self.to_string()]
    }
}

#[allow(unused)]
#[allow(clippy::wrong_self_convention)]
impl<T, P, X, const N: usize> SingleFieldPropsBuilder<T, P, X, N>
where
    T: Field<N> + PartialEq,
    P: Component + Clone,
    X: Clone + PartialEq,
{
    pub fn parent(mut self, parent: ComponentLink<P>) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn tag(mut self, tag: Option<X>) -> Self {
        self.tag = tag;
        self
    }

    pub fn label<S: SingleOrArrayLabels<N>>(mut self, label: S) -> Self {
        self.label = Some(label.into_array());
        self
    }

    pub fn size(mut self, size: [usize; N]) -> Self {
        self.size = Some(size);
        self
    }

    pub fn is_map(mut self, is_map: bool) -> Self {
        self.is_map = is_map;
        self
    }

    pub fn is_scrollable(mut self, is_scrollable: bool) -> Self {
        self.is_scrollable = is_scrollable;
        self
    }

    pub fn build(self) -> SingleFieldProps<T, P, X, N> {
        SingleFieldProps {
            value: self.value.unwrap(),
            parent: self.parent.unwrap(),
            tag: self.tag,
            size: self.size,
            label: self.label,
            is_map: self.is_map,
            is_scrollable: self.is_scrollable,
        }
    }
}

impl<T, P, X, const N: usize> Properties for SingleFieldProps<T, P, X, N>
where
    T: Field<N> + PartialEq,
    P: Component + Clone,
    X: Clone + PartialEq,
{
    type Builder = SingleFieldPropsBuilder<T, P, X, N>;

    fn builder() -> Self::Builder {
        SingleFieldPropsBuilder {
            value: None,
            parent: None,
            tag: None,
            label: None,
            size: None,
            is_map: false,
            is_scrollable: false,
        }
    }
}

pub enum SingleFieldMsg {
    Change(usize, ChangeData),
}

#[derive(Clone)]
pub struct SingleField<T, P, X, const N: usize>
where
    T: 'static + Field<N> + PartialEq,
    <P as Component>::Message: ParentMessage<T, X>,
    P: Component + Clone,
    X: 'static + Clone + PartialEq,
{
    link: ComponentLink<Self>,
    props: SingleFieldProps<T, P, X, N>,
    error: Option<String>,

    serialized: [String; N],

    node_ref: NodeRef,
    marker: PhantomData<T>,
}

fn set_height(field: &yew::web_sys::HtmlElement) {
    field.style().set_property("min-height", "").unwrap();
    field
        .style()
        .set_property("min-height", &format!("{}px", field.scroll_height()))
        .unwrap();
}

impl<T, P, X, const N: usize> Component for SingleField<T, P, X, N>
where
    T: 'static + Field<N> + PartialEq,
    <P as Component>::Message: ParentMessage<T, X>,
    P: Component + Clone,
    X: 'static + Clone + PartialEq,
{
    type Message = SingleFieldMsg;
    type Properties = SingleFieldProps<T, P, X, N>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SingleField {
            link,
            serialized: props.value.serialize(),
            props,
            error: None,
            node_ref: NodeRef::default(),
            marker: PhantomData,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.value != self.props.value {
            self.serialized = props.value.serialize();
            self.error = None;
        }

        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let SingleFieldMsg::Change(i, ChangeData::Value(s)) = msg {
            self.serialized[i] = s;

            match T::deserialize(
                self.serialized
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ) {
                Ok(t) => {
                    self.error = None;
                    self.props
                        .parent
                        .send_message(<<P as Component>::Message>::build_message(
                            t,
                            self.props.tag.clone(),
                        ));
                }
                Err(e) => self.error = Some(format!("{:?}", e)),
            }
        }

        true
    }

    fn view(&self) -> Html {
        let build_padding = |i| {
            let mut p = classes!("px-1");
            if i == 0 {
                p.push("ps-2");
            }
            if i == N - 1 {
                p.push("pe-2");
            }
            p
        };
        let size = match self.props.size {
            Some(s) => s,
            None => [12 / N; N],
        };
        let padding = match self.props.is_map {
            true => (0..N).map(|i| Some(build_padding(i))).collect::<Vec<_>>(),
            false => vec![None; N],
        };
        let label = match &self.props.label {
            Some(s) => s.iter().map(Option::Some).collect::<Vec<_>>(),
            None => vec![None; N],
        };
        let is_invalid = self.error.as_ref().map(|_| "is-invalid");
        let error = self
            .error
            .as_ref()
            .map(|e| html! { <div class="invalid-feedback">{ e }</div>});

        let (scrollable_class, resize_cb) = match self.props.is_scrollable {
            true => ("", Callback::noop()),
            false => (
                "no-scroll",
                Callback::Callback(Rc::new(|e: yew::events::InputData| {
                    use wasm_bindgen::JsCast;

                    let field = e
                        .event
                        .target()
                        .unwrap()
                        .dyn_ref::<yew::web_sys::HtmlElement>()
                        .unwrap()
                        .clone();
                    set_height(&field);
                })),
            ),
        };

        html! {
            <Row>
                {
                    for self.serialized.iter().zip(size.iter().zip(padding.iter())).zip(label.iter()).enumerate().map(|(i, ((v, (s, p)), l))| html! {
                        <Column xs=*s class=classes!(is_invalid, p).to_string()>
                            <div class="form-floating">
                                <textarea ref=self.node_ref.clone() oninput=resize_cb.clone() class=classes!("form-control", is_invalid, scrollable_class) value={ v.clone() } onchange=self.link.callback(move |e| SingleFieldMsg::Change(i, e))></textarea>
                                { l.map(|l| html! { <label>{ l }</label> }).unwrap_or_default() }
                            </div>
                        </Column>
                    })
                }
                {
                    error.unwrap_or_default()
                }
            </Row>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if !self.props.is_scrollable {
            if let Some(field) = self.node_ref.cast::<yew::web_sys::HtmlElement>() {
                set_height(&field);
            }
        }
    }
}

#[derive(Clone, Properties)]
pub struct SelectFieldProps<T, P, X = usize>
where
    T: Clone + ToString,
    P: Component + Clone,
    X: Clone,
{
    pub values: Vec<T>,
    #[prop_or_default]
    pub selected: Option<T>,
    pub parent: ComponentLink<P>,
    #[prop_or_default]
    pub allow_empty: bool,
    #[prop_or(None)]
    pub tag: Option<X>,
    #[prop_or(None)]
    pub label: Option<String>,
}

pub enum SelectFieldMsg {
    Change(ChangeData),
}

pub struct SelectField<T, P, X = ()>
where
    T: 'static + ToString + Clone + PartialEq,
    <P as Component>::Message: ParentMessage<Option<T>, X>,
    P: Component + Clone,
    X: 'static + Clone,
{
    link: ComponentLink<Self>,
    props: SelectFieldProps<T, P, X>,

    node_ref: NodeRef,
    marker: PhantomData<T>,
}

impl<T, P, X> Component for SelectField<T, P, X>
where
    T: 'static + ToString + Clone + PartialEq,
    <P as Component>::Message: ParentMessage<Option<T>, X>,
    P: Component + Clone,
    X: 'static + Clone,
{
    type Message = SelectFieldMsg;
    type Properties = SelectFieldProps<T, P, X>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SelectField {
            link,
            props,
            node_ref: NodeRef::default(),
            marker: PhantomData,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let SelectFieldMsg::Change(ChangeData::Select(s)) = msg {
            let effective_index = s.selected_index() - if self.props.allow_empty { 1 } else { 0 };
            let selected = match effective_index {
                x if x < 0 => None,
                v => Some(self.props.values[v as usize].clone()),
            };
            self.props
                .parent
                .send_message(<<P as Component>::Message>::build_message(
                    selected,
                    self.props.tag.clone(),
                ));
        }

        true
    }

    fn view(&self) -> Html {
        let selected = |v: Option<&T>| v == self.props.selected.as_ref();

        html! {
            <div class="form-floating">
                <select ref=self.node_ref.clone() class="form-select" onchange=self.link.callback(SelectFieldMsg::Change)>
                    {
                        if self.props.allow_empty {
                            html! { <option selected=selected(None)>{ "Default" }</option> }
                        } else {
                            html! {}
                        }
                    }
                    { for self.props.values.iter().map(|v| html!{ <option selected=selected(Some(v))>{ v.to_string() }</option> }) }
                </select>
                { self.props.label.as_ref().map(|l| html! { <label>{ l }</label> }).unwrap_or_default() }
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        // Without this the value in the UI won't change
        if let Some(field) = self.node_ref.cast::<yew::web_sys::HtmlSelectElement>() {
            field.set_value(
                &self
                    .props
                    .selected
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "Default".into()),
            );
        }
    }
}

impl<K, V> ParentMessage<(Key, K), K> for MapFieldMsg<K, V> {
    fn build_message((_, data): (Key, K), tag: Option<K>) -> Self {
        MapFieldMsg::ChangeKey(tag.unwrap(), data)
    }
}
impl<K, V> ParentMessage<(Value, V), K> for MapFieldMsg<K, V> {
    fn build_message((_, data): (Value, V), tag: Option<K>) -> Self {
        MapFieldMsg::ChangeValue(tag.unwrap(), data)
    }
}
impl<K, V> ParentMessage<(Key, Option<K>), K> for MapFieldMsg<K, V> {
    fn build_message((_, data): (Key, Option<K>), _tag: Option<K>) -> Self {
        MapFieldMsg::SetNewKey(data)
    }
}
impl<K, V> ParentMessage<(Value, Option<V>), K> for MapFieldMsg<K, V> {
    fn build_message((_, data): (Value, Option<V>), _tag: Option<K>) -> Self {
        MapFieldMsg::SetNewValue(data)
    }
}

macro_rules! impl_marker {
    ($type:ident) => {
        impl<F: Field<N>, const N: usize> Field<N> for ($type, F) {
            type DeserializeError = <F as Field<N>>::DeserializeError;

            fn deserialize(s: [&str; N]) -> Result<Self, Self::DeserializeError> {
                Ok(($type, F::deserialize(s)?))
            }

            fn serialize(&self) -> [String; N] {
                self.1.serialize()
            }
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Key;
impl_marker!(Key);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Value;
impl_marker!(Value);

#[derive(Clone)]
pub struct MapFieldProps<K, V, P, X, const KN: usize, const VN: usize>
where
    K: Field<KN> + PartialEq,
    V: Field<VN> + PartialEq,
    P: Component + Clone,
    X: Clone,
{
    pub map: BTreeMap<K, V>,
    pub parent: ComponentLink<P>,
    pub tag: Option<X>,
    pub label: Option<String>,
    pub key_label: Option<[String; KN]>,
    pub value_label: Option<[String; VN]>,
}

#[derive(Clone)]
pub struct MapFieldPropsBuilder<K, V, P: Component, X, const KN: usize, const VN: usize> {
    map: Option<BTreeMap<K, V>>,
    parent: Option<ComponentLink<P>>,
    tag: Option<X>,
    label: Option<String>,
    key_label: Option<[String; KN]>,
    value_label: Option<[String; VN]>,
}

#[allow(unused)]
#[allow(clippy::wrong_self_convention)]
impl<K, V, P, X, const KN: usize, const VN: usize> MapFieldPropsBuilder<K, V, P, X, KN, VN>
where
    K: Field<KN> + PartialEq,
    V: Field<VN> + PartialEq,
    P: Component + Clone,
    X: Clone,
{
    pub fn map(mut self, map: BTreeMap<K, V>) -> Self {
        self.map = Some(map);
        self
    }

    pub fn parent(mut self, parent: ComponentLink<P>) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn tag(mut self, tag: Option<X>) -> Self {
        self.tag = tag;
        self
    }

    pub fn label<S: AsRef<str>>(mut self, label: S) -> Self {
        self.label = Some(label.as_ref().to_string());
        self
    }

    pub fn key_label<S: SingleOrArrayLabels<KN>>(mut self, label: S) -> Self {
        self.key_label = Some(label.into_array());
        self
    }

    pub fn value_label<S: SingleOrArrayLabels<VN>>(mut self, label: S) -> Self {
        self.value_label = Some(label.into_array());
        self
    }

    pub fn build(self) -> MapFieldProps<K, V, P, X, KN, VN> {
        MapFieldProps {
            map: self.map.unwrap(),
            parent: self.parent.unwrap(),
            tag: self.tag,
            label: self.label,
            key_label: self.key_label,
            value_label: self.value_label,
        }
    }
}

impl<K, V, P, X, const KN: usize, const VN: usize> Properties for MapFieldProps<K, V, P, X, KN, VN>
where
    K: Field<KN> + PartialEq,
    V: Field<VN> + PartialEq,
    P: Component + Clone,
    X: Clone,
{
    type Builder = MapFieldPropsBuilder<K, V, P, X, KN, VN>;

    fn builder() -> Self::Builder {
        MapFieldPropsBuilder {
            map: None,
            parent: None,
            tag: None,
            label: None,
            key_label: None,
            value_label: None,
        }
    }
}

#[derive(Debug)]
pub enum MapFieldMsg<K, V> {
    ChangeKey(K, K),
    ChangeValue(K, V),
    RemoveKey(K),

    SetNewKey(Option<K>),
    SetNewValue(Option<V>),
    AddNew,
}

#[derive(Clone, Debug)]
pub enum MapUpdate<K, V> {
    Set(K, V),
    Remove(K),
}

#[derive(Clone)]
pub struct MapField<K, V, P, X, const KN: usize, const VN: usize>
where
    K: 'static + Field<KN> + std::cmp::Ord + PartialEq,
    V: 'static + Field<VN> + PartialEq,
    X: 'static + Clone,
    <P as Component>::Message: ParentMessage<MapUpdate<K, V>, X>,
    P: Component + Clone,
{
    link: ComponentLink<Self>,
    props: MapFieldProps<K, V, P, X, KN, VN>,

    new_key: Option<K>,
    new_value: Option<V>,

    marker: PhantomData<(K, V)>,
}

impl<K, V, P, X, const KN: usize, const VN: usize> Component for MapField<K, V, P, X, KN, VN>
where
    K: 'static + Field<KN> + std::cmp::Ord + PartialEq,
    V: 'static + Field<VN> + PartialEq,
    X: 'static + Clone,
    <P as Component>::Message: ParentMessage<MapUpdate<K, V>, X>,
    P: Component + Clone,
{
    type Message = MapFieldMsg<K, V>;
    type Properties = MapFieldProps<K, V, P, X, KN, VN>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MapField {
            link,
            props,

            new_key: None,
            new_value: None,

            marker: PhantomData,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MapFieldMsg::ChangeKey(old, new) => {
                let v = self.props.map.remove(&old).unwrap();
                self.props.parent.send_message_batch(vec![
                    ParentMessage::build_message(MapUpdate::Remove(old), self.props.tag.clone()),
                    ParentMessage::build_message(MapUpdate::Set(new, v), self.props.tag.clone()),
                ]);
            }
            MapFieldMsg::ChangeValue(k, new_value) => {
                self.props
                    .parent
                    .send_message(<P as Component>::Message::build_message(
                        MapUpdate::Set(k, new_value),
                        self.props.tag.clone(),
                    ));
            }
            MapFieldMsg::RemoveKey(k) => {
                self.props
                    .parent
                    .send_message(<P as Component>::Message::build_message(
                        MapUpdate::Remove(k),
                        self.props.tag.clone(),
                    ));
            }
            MapFieldMsg::SetNewKey(new_key) => self.new_key = new_key,
            MapFieldMsg::SetNewValue(new_value) => self.new_value = new_value,
            MapFieldMsg::AddNew => {
                if let (Some(k), Some(v)) = (&self.new_key, &self.new_value) {
                    self.props
                        .parent
                        .send_message(<P as Component>::Message::build_message(
                            MapUpdate::Set(k.clone(), v.clone()),
                            self.props.tag.clone(),
                        ));

                    self.new_key = None;
                    self.new_value = None;
                }
            }
        }

        true
    }

    fn view(&self) -> Html {
        #![allow(unused_parens)]

        let new_key = (Key, self.new_key.clone()).build_component(
            true,
            None,
            self.props.key_label.clone(),
            self.link.clone(),
        );
        let new_value = (Value, self.new_value.clone()).build_component(
            true,
            None,
            self.props.value_label.clone(),
            self.link.clone(),
        );

        html! {
            <div class="map-container">
                { self.props.label.as_ref().map(|l| html!{ <label class="form-label">{ l }</label> }).unwrap_or_default() }
                {
                    for self.props.map.iter().map(|(k, v)| {
                        let k_cloned = k.clone();

                        let key = (Key, k.clone()).build_component(true, Some(k.clone()), self.props.key_label.clone(), self.link.clone());
                        let value = (Value, v.clone()).build_component(true, Some(k.clone()), self.props.value_label.clone(), self.link.clone());

                        html! {
                            <Row class="px-1 d-flex align-items-stretch map-row">
                                <Column xs=4>
                                    { key }
                                </Column>
                                <Column xs=7>
                                    { value }
                                </Column>
                                <Column xs=1 class="p-0">
                                    <button type="button" class="btn-height-stretch btn btn-outline-secondary" onclick=self.link.callback_once(move |_| MapFieldMsg::RemoveKey(k_cloned))><i class="bi bi-trash"></i></button>
                                </Column>
                            </Row>
                        }
                    })
                }
                <Row class="px-1 d-flex align-items-stretch map-row">
                    <Column xs=4>
                        { new_key }
                    </Column>
                    <Column xs=7>
                        { new_value }
                    </Column>
                    <Column xs=1 class="p-0">
                        <button type="button" class="btn-height-stretch btn btn-outline-secondary" disabled=(self.new_key.is_none() || self.new_value.is_none()) onclick=self.link.callback_once(|_| MapFieldMsg::AddNew)><i class="bi bi-plus"></i></button>
                    </Column>
                </Row>
            </div>
        }
    }
}
