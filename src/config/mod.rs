mod configer;
mod schema;
pub mod shortcut;
//pub mod router;
pub use configer::load_config;
pub use configer::Config;
pub use configer::DBNodeConfig;
pub use shortcut::build_config_shortcut;
pub use shortcut::ConfigShortcut;
