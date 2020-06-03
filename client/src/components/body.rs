use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub props: Properties,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Msg {}

#[derive(Clone, Properties, Serialize, Deserialize, PartialEq)]
pub struct Properties {}

use super::bootstrap::{
    grid::{Col, Container, Row},
    navbar::{Nav, NavItem, Navbar, NavbarBrand, NavbarCollapse, NavbarCollapseToggler},
    progress::{Progress, ProgressBar},
};

impl Component for Model {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <Container size="fluid">
                <Row>
                    <Col>
                        <Navbar style="border-bottom: 1px solid #888;" expand="large"
                            brand={html! { <NavbarBrand>{"@Nova"}</NavbarBrand> }}>
                            <Nav>
                                <NavItem href="#" active=true>{"Home"}</NavItem>
                                <NavItem href="#">{"Next"}</NavItem>
                                <NavItem href="#">{"Last"}</NavItem>
                            </Nav>
                        </Navbar>
                    </Col>
                </Row>
            </Container>

            /*
            <div>
                <nav class="navbar navbar-expand-lg navbar-light bg-light">
                <a class="navbar-brand" href="#">{ "Navbar" }</a>
                <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav mr-auto">
                    <li class="nav-item active">
                        <a class="nav-link" href="#">{"Home"} <span class="sr-only">{ "(current)" }</span></a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{ "Link" }</a>
                    </li>
                    <li class="nav-item dropdown">
                        <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                        {"Dropdown"}
                        </a>
                        <div class="dropdown-menu" aria-labelledby="navbarDropdown">
                        <a class="dropdown-item" href="#">{"Action"}</a>
                        <a class="dropdown-item" href="#">{"Another action"}</a>
                        <div class="dropdown-divider"></div>
                        <a class="dropdown-item" href="#">{"Something else here"}</a>
                        </div>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">{"Disabled"}</a>
                    </li>
                    </ul>
                    <form class="form-inline my-2 my-lg-0">
                    <input class="form-control mr-sm-2" type="search" placeholder="Search" aria-label="Search"/>
                    <button class="btn btn-outline-success my-2 my-sm-0" type="submit">{"Search"}</button>
                    </form>
                </div>
                </nav>

            </div>
            */
        }
    }
}
