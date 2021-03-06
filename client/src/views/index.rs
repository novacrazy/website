use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

#[derive(Clone, Properties, PartialEq)]
pub struct IndexViewProps {
    #[prop_or(true)]
    pub running: bool,
}

impl PureComponent for IndexViewProps {
    fn render(&self) -> Html {
        html! {
            <div class={if self.running {""} else {"hidden"}}>{"Hello, Index!"}</div>
        }
    }
}

pub type IndexView = Pure<IndexViewProps>;
