#[cfg(debug_assertions)]
pub const BUILD_TYPE_STRING: &str = "debug";
#[cfg(not(debug_assertions))]
pub const BUILD_TYPE_STRING: &str = "release";
