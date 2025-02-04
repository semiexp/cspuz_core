pub mod arithmetic;
pub mod config;
pub mod csp;
mod csp_repr;
pub mod custom_constraints;
pub mod domain;

pub mod backend;
pub mod encoder;
pub mod integration;
pub mod norm_csp;
pub mod normalizer;

#[cfg(feature = "parser")]
pub mod csugar_cli;

#[cfg(feature = "parser")]
pub mod parser;

#[cfg(all(feature = "python-bindings", not(target_arch = "wasm32")))]
mod pyo3_binding;

pub mod sat;
mod util;

#[cfg(all(feature = "python-bindings", not(target_arch = "wasm32")))]
pub use pyo3_binding::cspuz_core;

#[cfg(test)]
mod test_util;
