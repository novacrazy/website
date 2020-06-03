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
            <>
                <Navbar style="border-bottom: 1px solid #888;" expand="small" bg="dark"
                    brand={html! { <NavbarBrand>{"@Nova"}</NavbarBrand> }}>
                    <Nav>
                        <NavItem href="#" active=true>{"Home"}</NavItem>
                        <NavItem href="#">{"Next"}</NavItem>
                        <NavItem href="#">{"Last"}</NavItem>
                    </Nav>
                </Navbar>

                <Container size="fluid">
                    <Row><Col>
                        <main>
                            {"Content"}
                        </main>
                    </Col></Row>
                </Container>
            </>
        }
    }
}
