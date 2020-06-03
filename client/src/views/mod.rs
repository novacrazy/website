use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::agent::{RouteAgentBridge, RouteRequest};
use yew_router::prelude::*;
use yew_router::switch::{AllowMissing, Permissive};
use yewtil::NeqAssign;

pub mod index;
pub mod portfolio;

pub struct MainView {
    pub link: ComponentLink<Self>,
    pub props: Properties,

    pub route: AppRoute,
    pub router_agent: RouteAgentBridge,
}

#[derive(Clone)]
pub enum MainMsg {
    Navigate(Route),
}

#[derive(Clone, Properties, Serialize, Deserialize, PartialEq)]
pub struct Properties {}

use crate::components::bootstrap::{
    grid::{Col, Container, Row},
    navbar::{Nav, NavItem, Navbar, NavbarBrand, NavbarCollapse, NavbarCollapseToggler},
    progress::{Progress, ProgressBar},
};

#[derive(Clone, Switch, PartialEq)]
pub enum AppRoute {
    #[to = "/#"]
    Index,

    #[to = "/portfolio"]
    Portfolio,

    #[to = "/not-found"]
    PageNotFound(Permissive<String>),
}

fn redirect(route: Route) -> AppRoute {
    match route.as_str() {
        "" | "/" | "/#" => AppRoute::Index,
        _ => AppRoute::PageNotFound(Permissive(Some(route.route))),
    }
}

impl Component for MainView {
    type Message = MainMsg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(MainMsg::Navigate);
        let mut router_agent = RouteAgentBridge::new(callback);
        router_agent.send(RouteRequest::GetCurrentRoute);

        MainView {
            link,
            props,
            route: AppRoute::Index,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MainMsg::Navigate(route) => {
                self.route = AppRoute::switch(route.clone()).unwrap_or_else(|| redirect(route));
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let navlink = |route| if route == self.route { "nav-link active" } else { "nav-link" }.to_owned();

        html! {
            <>
                <Navbar style="border-bottom: 1px solid #888;" expand="small" bg="dark"
                    brand={html! { <NavbarBrand>{"@Nova"}</NavbarBrand> }}>
                    <Nav>
                        <NavItem><RouterAnchor<AppRoute> route=AppRoute::Index classes={navlink(AppRoute::Index)}>
                            {"Home"}
                        </RouterAnchor<AppRoute>></NavItem>
                        <NavItem><RouterAnchor<AppRoute> route=AppRoute::Portfolio classes={navlink(AppRoute::Portfolio)}>
                            {"Portfolio"}
                        </RouterAnchor<AppRoute>></NavItem>
                    </Nav>
                </Navbar>

                <Router<AppRoute>
                    render = Router::render(|switch: AppRoute| {
                        use self::{index::IndexView, portfolio::PortfolioView};

                        match switch {
                            AppRoute::Index => html! {<IndexView/>},
                            AppRoute::Portfolio => html! {<PortfolioView/>},
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    })
                    redirect = Router::redirect(redirect)
                />
            </>
        }
    }
}
