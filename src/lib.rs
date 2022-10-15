mod config;
mod utils;

pub mod cli;

pub use config::{
    get_body_text, get_header_text, get_status_text, DiffConfig, DiffProfile, LoadConfig,
    RequestConfig, RequestProfile, ResponseProfile,
};
pub use utils::{diff_text, highlight_text};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}
