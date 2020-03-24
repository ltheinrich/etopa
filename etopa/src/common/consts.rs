//! Constant variables

use kern::meta;

pub const HELP: &str = "Help: TODO";
const CARGO_TOML: &str = include_str!("../../Cargo.toml");
static mut VERSION: &str = "";

/// Get version (unsafe, but should be safe unless VERSION is being modified)
pub fn version() -> &'static str {
    unsafe { VERSION }
}

/// Set version (unsafe!)
pub fn init_version() {
    unsafe {
        VERSION = meta::version(CARGO_TOML);
        println!("Etopa {} (c) 2020 Lennart Heinrich\n", VERSION);
    }
}
