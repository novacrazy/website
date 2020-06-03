use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

pub type NavbarBrand = Pure<NavbarBrandProps>;

#[derive(Clone, Properties, PartialEq)]
pub struct NavbarBrandProps {
    #[prop_or("#".to_owned())]
    pub href: String,

    pub children: Children,
}

impl PureComponent for NavbarBrandProps {
    fn render(&self) -> Html {
        html! { <a class="navbar-brand" href={self.href.as_str()}>{ self.children.render() }</a> }
    }
}
