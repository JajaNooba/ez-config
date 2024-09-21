//!
//! # ez_config: Simple env config library for Rust
//!
//! # Overview
//!
//! Ez_config allows to load config from env variables or `.env` file.
//!
//! # Example
//! ```
//! use ez_config::*;
//!
//! #[derive(Config)]
//! struct MyConfig {
//!     token: String,
//!     debug_mode: bool,
//!     some_numeber: i32
//! }
//!
//! fn main() {
//!     // load config from env
//!     let my_config = MyConfig::load();
//!
//!     // rest of the program
//! }
//! ```
//! In .env file
//! ```
//! TOKEN="my_secret_token"
//! DEBUG_MODE=false
//! SOME_NUMBER=123
//! ```

pub use ez_config_macro::Config;

pub trait Config {
    fn load() -> Self;
}
