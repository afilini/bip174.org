use yew::prelude::*;

use crate::app::{App, Msg};

pub enum NavbarMsg {
    ToggleBoxes,
    ToggleEdit,
}

#[derive(Debug)]
pub struct Navbar {
    link: ComponentLink<Self>,

    is_expanded: bool,
    is_editing: bool,
    checks_done: bool,

    network: bitcoin::Network, // TODO: custom enum
    app_link: ComponentLink<App>,
}

#[derive(Properties, Clone)]
pub struct NavbarProps {
    pub app_link: ComponentLink<App>,
    pub network: bitcoin::Network,
}

impl Navbar {
    fn emit_buttons(&self, classes: &'static str) -> Html {
        html! {
            <div class=classes role="group" aria-label="First group">
                <button type="button" class=("btn btn-outline-light", match self.is_expanded { true => Some("active"), _ => None }) onclick=self.link.callback(move |_| NavbarMsg::ToggleBoxes)><i class="bi bi-arrows-expand"></i></button>
                <button type="button" class=("btn btn-outline-light", match self.is_editing { true => Some("active"), _ => None }) onclick=self.link.callback(move |_| NavbarMsg::ToggleEdit)><i class="bi bi-pencil"></i></button>
                <button type="button" class="btn btn-outline-light"><i class="bi bi-ui-checks"></i></button>
            </div>
        }
    }

    fn toggle_boxes(&mut self) {
        self.is_expanded = !self.is_expanded;

        if !self.is_expanded && self.is_editing {
            self.is_editing = false;
            self.app_link.send_message(Msg::EditMode(self.is_editing));
        }

        self.app_link.send_message(Msg::ExpandBoxes(self.is_expanded));
    }

    fn toggle_edit(&mut self) {
        self.is_editing = !self.is_editing;

        if self.is_editing && !self.is_expanded {
            self.is_expanded = true;
            self.app_link.send_message(Msg::ExpandBoxes(self.is_expanded));
        }

        self.app_link.send_message(Msg::EditMode(self.is_editing));
    }
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = NavbarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            link,

            is_expanded: false,
            is_editing: false,
            checks_done: false,

            network: props.network,
            app_link: props.app_link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavbarMsg::ToggleBoxes => self.toggle_boxes(),
            NavbarMsg::ToggleEdit => self.toggle_edit(),
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar sticky-top navbar-expand-md navbar-dark bg-dark">
              <div class="container-fluid">
                <a class="navbar-brand" href="#">{ "BIP 174" }</a>


                { self.emit_buttons("btn-group me-2 d-block d-md-none position-absolute mobile-btns") }

                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarContent" aria-controls="navbarContent" aria-expanded="false" aria-label="Toggle navigation">
                  <span class="navbar-toggler-icon"></span>
                </button>

                <div class="collapse navbar-collapse" id="navbarContent">
                  <ul class="navbar-nav me-auto mb-2 mb-md-0">
                    <li class="nav-item">
                      <a class="nav-link active" aria-current="page" href="#">{ "Home" }</a>
                    </li>
                    <li class="nav-item">
                      <a class="nav-link" href="#">{ "About" }</a>
                    </li>
                    <li class="nav-item">
                      <a class="nav-link" href="#">{ "Support" }</a>
                    </li>
                    <li class="nav-item">
                      <a class="nav-link" href="#" tabindex="-1">{ "GitHub" }</a>
                    </li>
                  </ul>

                  { self.emit_buttons("btn-group me-2 d-none d-md-block" ) }

                  <div class="dropdown">
                    <button class="btn btn-primary dropdown-toggle" type="button" id="networkMenu" data-bs-toggle="dropdown" aria-expanded="false">
                        { "Mainnet" }
                    </button>

                    <ul class="dropdown-menu dropdown-menu-dark dropdown-menu-end" id="networkMenuContent" aria-labelledby="networkMenu">
                      <li><a class="dropdown-item active">{ "Mainnet" }</a></li>
                      <li><a class="dropdown-item">{ "Testnet" }</a></li>

                      <li><hr class="dropdown-divider"/></li>

                      <li><div class="dropdown-item">
                        <label for="customEndpoint" class="form-label">{ "Custom" }</label>
                        <input type="text" placeholder="http://localhost:5000/api" id="customEndpoint" class="form-control" aria-describedby="customEndpointHelp"/>

                        <div id="customEndpointHelp" class="form-text text-white">
                         { "Select a local or custom Esplora endpoint" }
                        </div>
                       </div></li>
                    </ul>
                  </div>

                </div>
              </div>
            </nav>
        }
    }
}
