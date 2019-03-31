#[cfg(not(feature = "steam"))]
#[path = "./api.rs"]
mod api;

#[cfg(feature = "steam")]
#[path = "./api_steam.rs"]
mod api;

pub use self::api::*;
