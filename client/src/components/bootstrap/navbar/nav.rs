use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

pub type Nav = Pure<NavProps>;
pub type NavItem = Pure<NavItemProps>;

#[derive(Clone, Properties, PartialEq)]
pub struct NavProps {
    pub children: ChildrenWithProps<NavItem>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct NavItemProps {
    #[prop_or_default]
    pub href: Option<String>,

    #[prop_or(false)]
    pub active: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for NavProps {
    fn render(&self) -> Html {
        html! {
            <ul class="navbar-nav mr-auto">{ self.children.render() }</ul>
        }
    }
}

impl PureComponent for NavItemProps {
    #[rustfmt::skip]
    fn render(&self) -> Html {
        let mut children = self.children.render();

        if let Some(ref href) = self.href {
            children = html! { <a class="nav-link" href={href.as_str()}>{ children }</a> };
        }

        let mut classes = Classes::new();
        classes.push("nav-item");

        if self.active   { classes.push("active"); }
        if self.disabled { classes.push("disabled"); }

        html! { <li class={classes}>{ children }</li> }
    }
}
