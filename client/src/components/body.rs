use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub props: Properties,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Msg {}

#[derive(Clone, Properties, Serialize, Deserialize, PartialEq)]
pub struct Properties {}

impl Component for Model {
    type Message = Msg;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { "Hello, World! 7" }
            </div>
        }
    }
}
