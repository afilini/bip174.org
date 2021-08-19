use yew::prelude::*;

use bitcoin::Network;

use crate::app::{App, AppMsg};

const ALL_NETWORKS: [Network; 4] = [
    Network::Bitcoin,
    Network::Testnet,
    Network::Regtest,
    Network::Signet,
];

pub struct Navbar {
    link: ComponentLink<Self>,
    props: NavbarProps,
}

#[derive(Clone, Properties)]
pub struct NavbarProps {
    pub network: Network,
    pub parent: ComponentLink<App>,
}

impl Component for Navbar {
    type Message = AppMsg;
    type Properties = NavbarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.parent.send_message(msg);
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <nav class="navbar sticky-top navbar-expand-md navbar-dark bg-dark">
                <div class="container-fluid">
                  <a class="navbar-brand" href="#">{ "BIP 174" }</a>

                    <div class="btn-group me-2 d-block d-md-none position-absolute" style="right: 72px" role="group" aria-label="First group">
                        <button type="button" class="btn btn-outline-light" onclick=self.link.callback(|_| AppMsg::Undo)><i class="bi bi-arrow-90deg-left"></i></button><button type="button" class="btn btn-outline-light" onclick=self.link.callback(|_| AppMsg::Redo)><i class="bi bi-arrow-90deg-right"></i></button>
                    </div>

                  <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarContent" aria-controls="navbarContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                  </button>

                  <div class="collapse navbar-collapse" id="navbarContent">
                    <ul class="navbar-nav me-auto mb-2 mb-md-0">
                      <li class="nav-item">
                        <a class="nav-link active" aria-current="page" href="#">{ "Home" }</a>
                      </li>
                      <li class="nav-item">
                        <a class="nav-link" href="#" data-bs-toggle="modal" data-bs-target="#aboutModal">{ "About" }</a>
                      </li>
                      // <li class="nav-item">
                      //   <a class="nav-link" href="#">{ "Help" }</a>
                      // </li>
                      <li class="nav-item">
                        <a class="nav-link" href="https://github.com/afilini/bip174.org" tabindex="-1">{ "GitHub" }</a>
                      </li>
                    </ul>

                    <div class="btn-group me-2 d-none d-md-block" role="group" aria-label="First group">
                        <button type="button" class="btn btn-outline-light" onclick=self.link.callback(|_| AppMsg::Undo)><i class="bi bi-arrow-90deg-left"></i></button><button type="button" class="btn btn-outline-light" onclick=self.link.callback(|_| AppMsg::Redo)><i class="bi bi-arrow-90deg-right"></i></button>
                    </div>

                    <div class="dropdown">
                      <button class="btn btn-primary dropdown-toggle" type="button" id="networkMenu" data-bs-toggle="dropdown" aria-expanded="false">
                        { first_letter_to_upper(&self.props.network.to_string()) }
                      </button>

                      <ul class="dropdown-menu dropdown-menu-dark dropdown-menu-end" id="networkMenuContent" aria-labelledby="networkMenu">
                      {
                        for ALL_NETWORKS.iter().map(|n| html! {
                            <li onclick=self.link.callback(move |_| AppMsg::SetNetwork(*n))>
                                <a class=classes!("dropdown-item", if n == &self.props.network { Some("active") } else { None })>{ first_letter_to_upper(&n.to_string()) }</a>
                            </li>
                        })
                      }
                      </ul>
                    </div>

                  </div>
                </div>
            </nav>

            <div class="modal fade" id="aboutModal" tabindex="-1" aria-labelledby="aboutModalLabel" aria-hidden="true">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="aboutModalLabel">{ "About" }</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <p>{ "This website is written in Rust and fully open-source: you can find the source code on " }<a href="https://github.com/afilini/bip174.org">{ "GitHub" }</a></p>
                            <p>{ "If you find a bug please file an issue and we'll try to help you!" }</p>
                            <p class="text-muted text-center pt-4 fst-italic">{ "Version: " }<span class="font-monospace">{ env!("CARGO_PKG_VERSION") }</span>{ " (" }<span class="font-monospace">{ env!("GIT_STATUS") }</span>{ ")" }</p>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{ "Close" }</button>
                        </div>
                    </div>
                </div>
            </div>
            </>
        }
    }
}

// https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn first_letter_to_upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
