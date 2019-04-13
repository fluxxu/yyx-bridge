#[cfg(all(not(feature = "steam"), not(feature = "fg")))]
#[path = "./api.rs"]
mod api;

#[cfg(feature = "steam")]
#[path = "./api_dll.rs"]
mod api;

#[cfg(feature = "fg")]
#[path = "./api_dll.rs"]
mod api;

pub use self::api::*;
