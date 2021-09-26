#[cfg(any(feature = "g0", feature = "g4"))]
pub mod g4;
#[cfg(any(feature = "g0", feature = "g4"))]
pub use g4::fdcan::RegisterBlock;

#[cfg(feature = "h7")]
pub mod h7;
pub use h7::fdcan::RegisterBlock;

pub mod generic;