use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

pub mod doom_fire;

#[derive(Clone, Properties, PartialEq)]
pub struct PortfolioViewProps {
    #[prop_or(true)]
    pub running: bool,
}

impl PureComponent for PortfolioViewProps {
    fn render(&self) -> Html {
        use crate::components::bootstrap::{
            grid::{Col, Container, Row},
            navbar::{Nav, NavItem, Navbar, NavbarBrand, NavbarCollapse, NavbarCollapseToggler},
            progress::{Progress, ProgressBar},
        };

        use doom_fire::DoomFire;

        html! {
            <div class={if self.running {""} else {"hidden"} }>
                <Container size="fluid">
                    <Row>
                        <Col>
                            <div>{"Hello, Portfolio!"}</div>
                        </Col>
                    </Row>
                    <Row>
                        <Col>
                            <DoomFire width=600, height=400 running={self.running}/>
                        </Col>
                    </Row>
                </Container>
            </div>
        }
    }
}

pub type PortfolioView = Pure<PortfolioViewProps>;
