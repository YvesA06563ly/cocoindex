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
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<indexing::PyIndexBuilder>()?;
    m.add_class::<pipeline::PyPipeline>()?;
    m.add_class::<storage::PyStorageConfig>()?;
    m.add_function(wrap_pyfunction!(indexing::create_index, m)?)?;
    m.add_function(wrap_pyfunction!(pipeline::run_pipeline, m)?)?;
    // expose version string so Python callers can sanity-check the native lib
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
