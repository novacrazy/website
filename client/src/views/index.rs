use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

#[derive(Clone, Properties, PartialEq)]
pub struct IndexViewProps {}

impl PureComponent for IndexViewProps {
    fn render(&self) -> Html {
        html! {
            <div>{"Hello, Index!"}</div>
        }
    }
}

pub type IndexView = Pure<IndexViewProps>;
