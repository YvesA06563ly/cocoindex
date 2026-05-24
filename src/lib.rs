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
///
/// Note to self: `__build_timestamp__` added so I can tell apart wheels
/// built on the same commit but at different times (e.g. after a toolchain
/// upgrade).
///
/// Note to self: `__build_target__` added so I can distinguish x86_64 vs
/// arm64 wheels when cross-compiling for Apple Silicon testing.
///
/// Note to self: `__build_profile__` is useful when I accidentally run
/// benchmarks against a debug wheel — caught me out more than once.
///
/// Note to self: `__rust_version__` added so I can track which toolchain
/// version was used — helps when debugging subtle codegen differences.
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
    // expose build timestamp so I can distinguish wheels built from the same commit
    m.add("__build_timestamp__", option_env!("BUILD_TIMESTAMP").unwrap_or("unknown"))?;
    // expose target triple so I can distinguish x86_64 vs arm64 wheels when cross-compiling
    m.add("__build_target__", env!("TARGET"))?;
    // expose rustc version used at build time — useful for tracking down codegen quirks
    // Note: CARGO_PKG_RUST_VERSION is the *minimum* required version, not the actual compiler
    // version used. Use RUSTC_VERSION env var set by build.rs instead.
    // TODO: wire up build.rs to set RUSTC_VERSION via `rustc --version` at build time
    m.add("__rust_version__", option_env!("RUSTC_VERSION").unwrap_or("unknown"))?;
    Ok(())
}
