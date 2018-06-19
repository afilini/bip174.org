use log::*;
use yew::classes;
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct ChildrenWrapper {
    #[prop_or_default]
    pub xs: Option<usize>,
    #[prop_or_default]
    pub sm: Option<usize>,
    #[prop_or_default]
    pub md: Option<usize>,
    #[prop_or_default]
    pub lg: Option<usize>,
    #[prop_or_default]
    pub xl: Option<usize>,
    #[prop_or_default]
    pub xxl: Option<usize>,

    #[prop_or(String::new())]
    pub class: String,

    pub children: Children,
}

macro_rules! wrapper_component {
    ($name:ident, $render:expr) => {
        pub struct $name {
            props: ChildrenWrapper,
        }

        impl Component for $name {
            type Message = ();
            type Properties = ChildrenWrapper;

            fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                $name { props }
            }

            fn change(&mut self, props: Self::Properties) -> ShouldRender {
                self.props = props;
                true
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }

            fn view(&self) -> Html {
                let classes = vec![
                    if self.props.class.is_empty() {
                        None
                    } else {
                        Some(self.props.class.clone())
                    },
                    self.props.xs.map(|v| format!("col-{}", v)),
                    self.props.sm.map(|v| format!("col-sm-{}", v)),
                    self.props.md.map(|v| format!("col-md-{}", v)),
                    self.props.lg.map(|v| format!("col-lg-{}", v)),
                    self.props.xl.map(|v| format!("col-xl-{}", v)),
                    self.props.xxl.map(|v| format!("col-xxl-{}", v)),
                ];
                ($render)(self, classes)
            }
        }
    };
}

wrapper_component!(Container, |s: &Container, c: Vec<Option<String>>| {
    html! { <div class=classes!("container", c)> { s.props.children.clone() } </div> }
});
wrapper_component!(
    ContainerFluid,
    |s: &ContainerFluid, c: Vec<Option<String>>| {
        html! { <div class=classes!("container-fluid", c)> { s.props.children.clone() } </div> }
    }
);
wrapper_component!(Row, |s: &Row, c: Vec<Option<String>>| {
    html! { <div class=classes!("row", c)> { s.props.children.clone() } </div> }
});
wrapper_component!(Column, |s: &Column, c: Vec<Option<String>>| {
    html! { <div class=classes!("column", c)> { s.props.children.clone() } </div> }
});
