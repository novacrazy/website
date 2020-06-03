use yew::virtual_dom::{Transformer, VComp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Background {
    None,
    Success,
    Info,
    Warning,
    Danger,
    Primary,
    Light,
    Dark,
}

impl Background {
    pub fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Background::None => return None,
            Background::Success => "bg-success",
            Background::Info => "bg-info",
            Background::Warning => "bg-warning",
            Background::Danger => "bg-danger",
            Background::Primary => "bg-primary",
            Background::Light => "bg-light",
            Background::Dark => "bg-dark",
        })
    }
}

impl Default for Background {
    fn default() -> Self {
        Background::None
    }
}

impl Transformer<&str, Background> for VComp {
    fn transform(from: &str) -> Background {
        match from {
            "bg-success" | "success" => Background::Success,
            "bg-info" | "info" => Background::Info,
            "bg-warning" | "warning" => Background::Warning,
            "bg-danger" | "danger" => Background::Danger,
            "bg-primary" | "primary" => Background::Primary,
            "bg-light" | "light" => Background::Light,
            "bg-dark" | "dark" => Background::Dark,
            _ => Background::None,
        }
    }
}
