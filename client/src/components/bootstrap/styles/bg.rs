#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Background {
    None,
    Success,
    Info,
    Warning,
    Danger,
}

impl Background {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Background::None => return None,
            Background::Success => "bg-success",
            Background::Info => "bg-info",
            Background::Warning => "bg-warning",
            Background::Danger => "bg-danger",
        })
    }
}

impl Default for Background {
    fn default() -> Self {
        Background::None
    }
}

use yew::virtual_dom::{Transformer, VComp};

impl Transformer<&str, Background> for VComp {
    fn transform(from: &str) -> Background {
        match from {
            "bg-success" | "success" => Background::Success,
            "bg-info" | "info" => Background::Info,
            "bg-warning" | "warning" => Background::Warning,
            "bg-danger" | "danger" => Background::Danger,
            _ => Background::None,
        }
    }
}
