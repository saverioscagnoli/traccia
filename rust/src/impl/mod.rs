#[cfg(not(feature = "blocking"))]
pub mod r#async;

#[cfg(feature = "blocking")]
pub mod blocking;
