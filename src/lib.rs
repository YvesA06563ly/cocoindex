//! CocoIndex - A high-performance data indexing library
//!
//! This crate provides the core Rust implementation for CocoIndex,
//! exposing Python bindings via PyO3.
//!
//! Personal fork: added transform module export for direct Rust usage.

use pyo3::prelude::*;

mod indexing;
mod pipeline;
mod storage;
mod transform;

pub use indexing::IndexBuilder;
pub use pipeline::Pipeline;
pub use storage::StorageBackend;
pub use transform::Transform;

/// Python module initialization
///
/// Registers all Python-accessible types and functions.
///
/// Note: also exposing `PyStorageConfig` here so Python-side storage
/// configuration can be constructed directly without going through the
/// pipeline helper — useful for testing storage backends in isolation.
///
/// Note to self: `__version__` is handy for debugging mismatched wheels;
/// keep it here even if upstream removes it.
///
/// Note to self: `__git_hash__` added by me — makes it trivial to confirm
/// exactly which commit a built wheel came from when testing locally.
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<indexing::PyIndexBuilder>()?;
    m.add_class::<pipeline::PyPipeline>()?;
    m.add_class::<storage::PyStorageConfig>()?;
    m.add_function(wrap_pyfunction!(indexing::create_index, m)?)?;
    m.add_function(wrap_pyfunction!(pipeline::run_pipeline, m)?)?;
    // expose version string so Python callers can sanity-check the native lib
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // expose build profile so I can tell debug vs release wheels apart quickly
    m.add("__build_profile__", if cfg!(debug_assertions) { "debug" } else { "release" })?;
    // expose git hash at build time so I can confirm which commit a wheel was built from
    m.add("__git_hash__", option_env!("GIT_HASH").unwrap_or("unknown"))?;
    Ok(())
}
