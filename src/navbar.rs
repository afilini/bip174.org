use yew::prelude::*;

use crate::app::{App, Msg};

#[derive(Clone, Debug, PartialEq)]
pub enum NetworkSettings {
    Standard(bitcoin::Network),
    Custom(bitcoin::Network, String),
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self::Standard(bitcoin::Network::Bitcoin)
    }
}

impl std::ops::Deref for NetworkSettings {
    type Target = bitcoin::Network;

    fn deref(&self) -> &Self::Target {
        match &self {
            &NetworkSettings::Standard(network) => network,
            &NetworkSettings::Custom(network, _) => network,
        }
    }
}

impl std::fmt::Display for NetworkSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkSettings::Standard(bitcoin::Network::Bitcoin) => write!(f, "Mainnet"),
            NetworkSettings::Standard(bitcoin::Network::Testnet) => write!(f, "Testnet"),
            NetworkSettings::Standard(bitcoin::Network::Regtest) => write!(f, "Regtest"),
            NetworkSettings::Custom(_, _) => write!(f, "Custom"),
        }
    }
}

pub enum NavbarMsg {
    ToggleBoxes,
    ToggleEdit,
    NetworkChanged(NetworkSettings),
}

#[derive(Debug)]
pub struct Navbar {
    link: ComponentLink<Self>,

    is_expanded: bool,
    is_editing: bool,
    checks_done: bool,

    network: NetworkSettings,
    app_link: ComponentLink<App>,
}

#[derive(Properties, Clone)]
pub struct NavbarProps {
    pub app_link: ComponentLink<App>,
    pub network: NetworkSettings,
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

    fn emit_network(&self) -> Html {
        let mainnet_settings = NetworkSettings::Standard(bitcoin::Network::Bitcoin);
        let testnet_settings = NetworkSettings::Standard(bitcoin::Network::Testnet);
        let regtest_settings = NetworkSettings::Standard(bitcoin::Network::Regtest);

        let mainnet_active = if &self.network == &mainnet_settings { "active" } else { "" };
        let testnet_active = if &self.network == &testnet_settings { "active" } else { "" };
        let regtest_active = if &self.network == &regtest_settings { "active" } else { "" };
        html! {
          <div class="dropdown">
            <button class="btn btn-primary dropdown-toggle" type="button" id="networkMenu" data-bs-toggle="dropdown" aria-expanded="false">
                {
                    format!("{}", self.network)
                }
            </button>

            <ul class="dropdown-menu dropdown-menu-dark dropdown-menu-end" id="networkMenuContent" aria-labelledby="networkMenu">
            <li>
                <a class=("dropdown-item", {mainnet_active})
                    onclick=self.link.callback(move |_| NavbarMsg::NetworkChanged(mainnet_settings.clone()))>
                    { "Mainnet" }
                </a>
            </li>
            <li>
                <a class=("dropdown-item", {testnet_active})
                    onclick=self.link.callback(move |_| NavbarMsg::NetworkChanged(testnet_settings.clone()))>
                    { "Testnet" }
                </a>
            </li>
            <li>
                <a class=("dropdown-item", {regtest_active})
                    onclick=self.link.callback(move |_| NavbarMsg::NetworkChanged(regtest_settings.clone()))>
                    { "Regtest" }
                </a>
            </li>

              // TODO: custom endpoint
              /*
              <li><hr class="dropdown-divider"/></li>

              <li><div class="dropdown-item">
                <label for="customEndpoint" class="form-label">{ "Custom" }</label>
                <input type="text" placeholder="http://localhost:5000/api" id="customEndpoint" class="form-control" aria-describedby="customEndpointHelp"/>

                <div id="customEndpointHelp" class="form-text text-white">
                 { "Select a local or custom Esplora endpoint" }
                </div>
               </div></li>
              */
            </ul>
          </div>
        }
    }

    fn toggle_boxes(&mut self) {
        self.is_expanded = !self.is_expanded;

        if !self.is_expanded && self.is_editing {
            self.is_editing = false;
            self.app_link.send_message(Msg::EditMode(self.is_editing));
        }

        self.app_link
            .send_message(Msg::ExpandBoxes(self.is_expanded));
    }

    fn toggle_edit(&mut self) {
        self.is_editing = !self.is_editing;

        if self.is_editing && !self.is_expanded {
            self.is_expanded = true;
            self.app_link
                .send_message(Msg::ExpandBoxes(self.is_expanded));
        }

        self.app_link.send_message(Msg::EditMode(self.is_editing));
    }

    fn network_changed(&mut self, network: NetworkSettings) {
        self.network = network;
        self.app_link
            .send_message(Msg::NetworkChanged(self.network.clone()));
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

            network: NetworkSettings::default(),
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
            NavbarMsg::NetworkChanged(network) => self.network_changed(network),
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

                  { self.emit_network() }
                </div>
              </div>
            </nav>
        }
    }
}
