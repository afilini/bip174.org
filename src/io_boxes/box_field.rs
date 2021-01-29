use std::collections::BTreeMap;
use yew::prelude::*;

#[derive(Clone)]
pub enum BoxFieldValue {
    Single(Option<String>),
    Group(Vec<(&'static str, Option<String>)>),
    Map(BTreeMap<String, String>),
    BooleanSummary(BTreeMap<&'static str, Option<String>>),
}

#[derive(Clone, Properties)]
pub struct BoxField {
    pub title: &'static str,
    pub value: BoxFieldValue,
    pub expanded: bool,
}

impl Component for BoxField {
    type Message = ();
    type Properties = BoxField;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        BoxField {
            title: props.title,
            value: props.value,
            expanded: props.expanded,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.value = props.value;
        self.expanded = props.expanded;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if self.expanded {
            html! {
                <div>
                    <div class="card-body py-2 d-flex">
                        <span class="col-5 fw-bold">{ self.title }</span>
                    </div>
                        { self.value.emit_long_html() }
                </div>
            }
        } else {
            html! {
                <div>
                    <div class="card-body py-2 d-flex">
                        <span class="col-5 fw-bold">{ self.title }</span>
                        <div class="col-7 font-monospace">
                            { self.value.emit_short_html() }
                        </div>
                    </div>
                </div>
            }
        }
    }
}

impl BoxFieldValue {
    fn emit_short_html(&self) -> Html {
        use BoxFieldValue::*;
        match self {
            Single(string) => {
                let class = match string {
                    Some(_) => "",
                    None => "text-muted",
                };

                html! { <span class=class>{ string.as_ref().unwrap_or(&"default".to_string()) }</span> }
            }
            Group(vec) => {
                html! {
                    for vec.iter().enumerate().map(|(index, (title, opt))| {
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
                    })
                }
            }
            Map(map) => {
                html! { <span>{ map.len().to_string() }</span> }
            }
            BooleanSummary(map) => {
                html! { <span>{ map.iter().map(|(_, v)| v).any(|v| v.is_some()) }</span> }
            }
        }
    }

    fn emit_long_html(&self) -> Html {
        use BoxFieldValue::*;
        match self {
            // FIXME: string should be on same line as title
            Single(string) => {
                let class = match string {
                    Some(_) => "",
                    None => "text-muted",
                };

                html! {
                    <div class="card-body py-2 d-flex">
                        <span class=("col-5 px-3 font-monospace", class)>{ string.as_ref().unwrap_or(&"default".to_string()) }</span>
                    </div>
                }
            }
            Group(vec) => {
                html! {
                    for vec.iter().map(|(title, opt)| {
                        let class = match opt {
                            Some(_) => "",
                            None => "text-muted",
                        };

                        html!{
                            <div class="card-body py-2 d-flex">
                                <span class="col-5 px-3 font-monospace">{title}</span>
                                <span class=("col-7 font-monospace", class)>{opt.as_ref().unwrap_or(&"none".to_string())}</span>
                            </div>
                        }
                    })
                }
            }
            Map(map) => {
                html! {
                    for map.iter().map(|(k, v)| {
                        html!{
                            <div class="card-body py-2 d-flex">
                                <span class="col-5 px-3 font-monospace">{k}</span>
                                <span class="col-7 font-monospace">{v}</span>
                            </div>
                        }
                    })
                }
            }
            BooleanSummary(map) => {
                html! {
                    for map.iter().map(|(k, v)| {
                        let class = match v {
                            Some(_) => "",
                            None => "text-muted",
                        };
                        html!{
                            <div class="card-body py-2 d-flex">
                                <span class="col-5 px-3 font-monospace">{k}</span>
                                <span class=("col-7 font-monospace", class)>
                                    {v.as_ref().unwrap_or(&"none".to_string())}
                                </span>
                            </div>
                        }
                    })
                }
            }
        }
    }
}
