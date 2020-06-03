use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewtil::{NeqAssign, Pure, PureComponent};

use super::styles::bg::Background;

pub type Progress = Pure<ProgressProps>;
pub type ProgressBar = Pure<ProgressBarProps>;

#[derive(Clone, Properties, PartialEq)]
pub struct ProgressProps {
    pub children: ChildrenWithProps<ProgressBar>,
}

impl PureComponent for ProgressProps {
    fn render(&self) -> Html {
        // sort progress bars by progress so they overlap correctly
        let mut progress_bars = self.children.to_vec();
        progress_bars.sort_by_key(|child| float_ord::FloatOrd(child.props.progress().1));

        html! { <div class="progress">{ for progress_bars.into_iter() }</div> }
    }
}

#[derive(Clone, Copy, Properties, PartialEq)]
pub struct ProgressBarProps {
    #[prop_or(0.0)]
    pub min: f32,

    #[prop_or(1.0)]
    pub max: f32,

    pub cur: f32,

    #[prop_or_default]
    pub bg: Background,

    #[prop_or(false)]
    pub striped: bool,

    #[prop_or(false)]
    pub animated: bool,
}

impl ProgressBarProps {
    #[inline]
    pub fn progress(&self) -> (f32, f32, f32) {
        let ProgressBarProps { min, max, cur, .. } = *self;

        let new_min = min.min(max);
        let new_max = max.max(min);

        let progress = cur.min(new_max).max(new_min) / (new_max - new_min);

        (new_min, progress, new_max)
    }
}

impl PureComponent for ProgressBarProps {
    #[rustfmt::skip]
    fn render(&self) -> Html {
        let (min, progress, max) = self.progress();

        let ProgressBarProps { cur, bg, striped, animated, .. } = *self;

        let mut classes = Classes::new();
        classes.push("progress-bar");

        bg.as_str().map(|bg| classes.push(bg));

        if striped  { classes.push("progress-bar-striped"); }
        if animated { classes.push("progress-bar-animated"); }

        html! {
            <div
                class={classes}
                role="progressbar"
                style={format!("width: {}%", progress * 100.0)}
                aria-valuenow={cur}
                aria-valuemin={min}
                aria-valuemax={max}
            />
        }
    }
}
