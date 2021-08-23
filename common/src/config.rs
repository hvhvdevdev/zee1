use alloc::string::String;

/// Struct for storing configuration information.
#[derive(Debug)]
pub struct RunConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub mod_root: String,
}
