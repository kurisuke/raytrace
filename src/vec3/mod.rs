#[cfg(target_feature = "sse2")]
pub mod sse2;

pub mod scalar;

#[cfg(target_feature = "sse2")]
pub use self::sse2::*;

#[cfg(not(target_feature = "sse2"))]
pub use self::scalar::*;
