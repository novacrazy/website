use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

#[derive(Clone, Properties, PartialEq)]
pub struct AboutViewProps {
    #[prop_or(true)]
    pub running: bool,
}

impl PureComponent for AboutViewProps {
    fn render(&self) -> Html {
        html! {
            <div class={if self.running {""} else {"hidden"}}>{"Hello, About!"}</div>
        }
    }
}

pub type AboutView = Pure<AboutViewProps>;
