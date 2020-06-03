use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

#[derive(Clone, Properties, PartialEq)]
pub struct PortfolioViewProps {}

impl PureComponent for PortfolioViewProps {
    fn render(&self) -> Html {
        html! {
            <div>{"Hello, Portfolio!"}</div>
        }
    }
}

pub type PortfolioView = Pure<PortfolioViewProps>;
