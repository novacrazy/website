use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

#[derive(Clone, Properties, PartialEq)]
pub struct MainViewProps {}

impl PureComponent for MainViewProps {
    fn render(&self) -> Html {
        html! {
            <div>{"Hello, Main!"}</div>
        }
    }
}

pub type MainView = Pure<MainViewProps>;
