#![forbid(unsafe_op_in_unsafe_fn)]

#[macro_use]
mod byond;
#[allow(dead_code)]
mod error;

#[cfg(feature = "jobs")]
mod jobs;

#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "regexp")]
pub mod regexp;
#[cfg(feature = "transliteration")]
pub mod transliteration;

#[cfg(not(target_pointer_width = "32"))]
compile_error!("rust-utils must be compiled for a 32-bit target");
