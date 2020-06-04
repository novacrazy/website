use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

pub mod doom_fire;

#[derive(Clone, Properties, PartialEq)]
pub struct PortfolioViewProps {}

impl PureComponent for PortfolioViewProps {
    fn render(&self) -> Html {
        use crate::components::bootstrap::{
            grid::{Col, Container, Row},
            navbar::{Nav, NavItem, Navbar, NavbarBrand, NavbarCollapse, NavbarCollapseToggler},
            progress::{Progress, ProgressBar},
        };

        use doom_fire::DoomFire;

        html! {
            <Container size="fluid">
                <Row>
                    <Col>
                        <div>{"Hello, Portfolio!"}</div>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <DoomFire width=600, height=400/>
                    </Col>
                </Row>
            </Container>
        }
    }
}

pub type PortfolioView = Pure<PortfolioViewProps>;
