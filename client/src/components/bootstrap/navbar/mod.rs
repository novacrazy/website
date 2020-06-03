use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::{Transformer, VComp};
use yewtil::{NeqAssign, Pure, PureComponent};

pub mod brand;
pub mod collapse;
pub mod nav;

pub use self::brand::NavbarBrand;
pub use self::collapse::{NavbarCollapse, NavbarCollapseToggler};
pub use self::nav::{Nav, NavItem};

use super::styles::bg::Background;

pub struct Navbar {
    pub link: ComponentLink<Self>,
    pub props: NavbarProps,

    pub collapsed: bool,
}

#[derive(Clone, Copy)]
pub enum NavbarMsg {
    SetCollapse(bool),
}

#[derive(Clone, Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or(NavbarTheme::Dark)]
    pub theme: NavbarTheme,

    #[prop_or_default]
    pub expand: NavbarSize,

    #[prop_or_default]
    pub bg: Background,

    #[prop_or_default]
    pub style: String,

    #[prop_or_default]
    pub brand: Html,
}

impl Component for Navbar {
    type Properties = NavbarProps;
    type Message = NavbarMsg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            props,
            link,
            collapsed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavbarMsg::SetCollapse(value) => {
                self.collapsed = value;
                return true;
            }
        }
    }

    fn change(&mut self, new: Self::Properties) -> ShouldRender {
        self.props.neq_assign(new)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::new();
        classes.push("navbar flex-row");

        classes.push(match self.props.theme {
            NavbarTheme::Light => "navbar-light",
            NavbarTheme::Dark => "navbar-dark",
        });

        classes.push(match self.props.expand {
            NavbarSize::Default => "navbar-expand",
            NavbarSize::Small => "navbar-expand-sm",
            NavbarSize::Medium => "navbar-expand-md",
            NavbarSize::Large => "navbar-expand-lg",
            NavbarSize::ExtraLarge => "navbar-expand-xl",
        });

        self.props.bg.as_str().map(|bg| classes.push(bg));

        html! {
            <header class={classes} style={&self.props.style}>
                { self.props.brand.clone() }
                <NavbarCollapseToggler on_toggle={self.link.callback(|value| NavbarMsg::SetCollapse(value))}/>
                <NavbarCollapse collapsed={self.collapsed}>
                    { self.props.children.render() }
                </NavbarCollapse>
            </header>
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NavbarSize {
    Default,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl Default for NavbarSize {
    fn default() -> Self {
        NavbarSize::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NavbarTheme {
    Light,
    Dark,
}

impl Transformer<&str, NavbarSize> for VComp {
    fn transform(from: &str) -> NavbarSize {
        match from {
            "small" | "sm" => NavbarSize::Small,
            "medium" | "md" => NavbarSize::Medium,
            "large" | "lg" => NavbarSize::Large,
            "extra-large" | "xl" => NavbarSize::ExtraLarge,
            _ => return NavbarSize::Default,
        }
    }
}

impl Transformer<&str, NavbarTheme> for VComp {
    fn transform(from: &str) -> NavbarTheme {
        match from {
            "light" => NavbarTheme::Light,
            _ => NavbarTheme::Dark,
        }
    }
}
