use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::{Transformer, VComp};
use yewtil::{NeqAssign, Pure, PureComponent};

pub type Container = Pure<ContainerProps>;
pub type Row = Pure<RowProps>;
pub type Col = Pure<ColProps>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContainerSize {
    Normal,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Fluid,
}

impl ContainerSize {
    pub fn to_class(self) -> &'static str {
        match self {
            ContainerSize::Normal => "container",
            ContainerSize::Small => "container-sm",
            ContainerSize::Medium => "container-md",
            ContainerSize::Large => "container-lg",
            ContainerSize::ExtraLarge => "container-xl",
            ContainerSize::Fluid => "container-fluid",
        }
    }
}

impl Default for ContainerSize {
    fn default() -> Self {
        ContainerSize::Normal
    }
}

impl Transformer<&str, ContainerSize> for VComp {
    fn transform(from: &str) -> ContainerSize {
        match from {
            "container-sm" | "small" => ContainerSize::Small,
            "container-md" | "medium" => ContainerSize::Medium,
            "container-lg" | "large" => ContainerSize::Large,
            "container-xl" | "xl" | "extra-large" => ContainerSize::ExtraLarge,
            "container-fluid" | "fluid" => ContainerSize::Fluid,
            _ => ContainerSize::Normal,
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ContainerProps {
    pub children: ChildrenWithProps<Row>,

    #[prop_or_default]
    pub size: ContainerSize,
}

#[derive(Clone, Properties, PartialEq)]
pub struct RowProps {
    pub children: ChildrenWithProps<Col>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct ColProps {
    pub children: Children,
}

impl PureComponent for ContainerProps {
    fn render(&self) -> Html {
        html! { <div class={self.size.to_class()}>{ self.children.render() }</div> }
    }
}

impl PureComponent for RowProps {
    fn render(&self) -> Html {
        html! { <div class="row">{ self.children.render() }</div> }
    }
}

impl PureComponent for ColProps {
    fn render(&self) -> Html {
        html! { <div class="col">{ self.children.render() }</div> }
    }
}
