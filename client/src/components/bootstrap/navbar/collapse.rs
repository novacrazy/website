use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

pub type NavbarCollapse = Pure<NavbarCollapseProps>;

#[derive(Clone, Properties, PartialEq)]
pub struct NavbarCollapseProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub collapsed: bool,

    #[prop_or_default]
    pub id: Option<String>,
}

impl PureComponent for NavbarCollapseProps {
    fn render(&self) -> Html {
        let mut classes = Classes::new();
        classes.push("navbar-collapse");
        classes.push("collapse");

        if !self.collapsed {
            classes.push("show");
        }

        let id = self.id.as_ref().map(|id| id.as_str()).unwrap_or("navbar-collapse");

        html! {
            <div class={classes} id={id}>{ self.children.render() }</div>
        }
    }
}

/////////////////

pub struct NavbarCollapseToggler {
    pub link: ComponentLink<Self>,
    pub props: NavbarCollapseTogglerProps,

    pub collapsed: bool,
}

#[derive(Clone, Copy)]
pub enum NavbarCollapseTogglerMsg {
    Toggle,
}

#[derive(Clone, Properties, PartialEq)]
pub struct NavbarCollapseTogglerProps {
    pub on_toggle: Callback<bool>,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or_default]
    pub controls: Option<String>,
}

impl Component for NavbarCollapseToggler {
    type Message = NavbarCollapseTogglerMsg;
    type Properties = NavbarCollapseTogglerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavbarCollapseToggler {
            props,
            link,
            collapsed: false,
        }
    }

    fn change(&mut self, new: Self::Properties) -> ShouldRender {
        self.props.neq_assign(new)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavbarCollapseTogglerMsg::Toggle => {
                self.collapsed = !self.collapsed;

                self.props.on_toggle.emit(self.collapsed);

                return true;
            }
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::new();
        classes.push("navbar-toggler");

        if self.collapsed {
            classes.push("collapsed");
        }

        let label = self.props.label.as_ref().map(|label| label.as_str()).unwrap_or("Toggle Navbar");
        let controls = self.props.controls.as_ref().map(|controls| controls.as_str()).unwrap_or("");

        html! {
            <button type="button" class={classes}
                aria-controls={controls}
                aria-expanded={!self.collapsed}
                aria-label={label}
                onclick={self.link.callback(|_| NavbarCollapseTogglerMsg::Toggle)}
            >
                <span class="navbar-toggler-icon"/>
            </button>
        }
    }
}
