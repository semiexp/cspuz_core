#![allow(static_mut_refs)] // TODO: remove this

use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use cspuz_core::config::Config;
use cspuz_core::csugar_cli::csugar_cli;
use cspuz_core::sat::{Backend, OrderEncodingLinearMode};

#[pyclass(name = "Config")]
#[derive(Clone)]
struct PyConfig {
    config: Config,
}

#[pymethods]
impl PyConfig {
    #[new]
    const fn new() -> PyConfig {
        PyConfig {
            config: Config::initial_default(),
        }
    }

    #[getter]
    fn get_use_constant_folding(&self) -> PyResult<bool> {
        Ok(self.config.use_constant_folding)
    }

    #[setter]
    fn set_use_constant_folding(&mut self, value: bool) -> PyResult<()> {
        self.config.use_constant_folding = value;
        Ok(())
    }

    #[getter]
    fn get_use_constant_propagation(&self) -> PyResult<bool> {
        Ok(self.config.use_constant_propagation)
    }

    #[setter]
    fn set_use_constant_propagation(&mut self, value: bool) -> PyResult<()> {
        self.config.use_constant_propagation = value;
        Ok(())
    }

    #[getter]
    fn get_use_norm_domain_refinement(&self) -> PyResult<bool> {
        Ok(self.config.use_norm_domain_refinement)
    }

    #[setter]
    fn set_use_norm_domain_refinement(&mut self, value: bool) -> PyResult<()> {
        self.config.use_norm_domain_refinement = value;
        Ok(())
    }

    #[getter]
    fn get_domain_product_threshold(&self) -> PyResult<i32> {
        Ok(self.config.domain_product_threshold as i32)
    }

    #[setter]
    fn set_domain_product_threshold(&mut self, value: i32) -> PyResult<()> {
        assert!(value >= 0);
        self.config.domain_product_threshold = value as usize;
        Ok(())
    }

    #[getter]
    fn get_native_linear_encoding_terms(&self) -> PyResult<i32> {
        Ok(self.config.native_linear_encoding_terms as i32)
    }

    #[setter]
    fn set_native_linear_encoding_terms(&mut self, value: i32) -> PyResult<()> {
        assert!(value >= 0);
        self.config.native_linear_encoding_terms = value as usize;
        Ok(())
    }

    #[getter]
    fn get_native_linear_encoding_domain_product_threshold(&self) -> PyResult<i32> {
        Ok(self.config.native_linear_encoding_domain_product_threshold as i32)
    }

    #[setter]
    fn set_native_linear_encoding_domain_product_threshold(&mut self, value: i32) -> PyResult<()> {
        assert!(value >= 0);
        self.config.native_linear_encoding_domain_product_threshold = value as usize;
        Ok(())
    }

    #[getter]
    fn get_use_direct_encoding(&self) -> PyResult<bool> {
        Ok(self.config.use_direct_encoding)
    }

    #[setter]
    fn set_use_direct_encoding(&mut self, value: bool) -> PyResult<()> {
        self.config.use_direct_encoding = value;
        Ok(())
    }

    #[getter]
    fn get_merge_equivalent_variables(&self) -> PyResult<bool> {
        Ok(self.config.merge_equivalent_variables)
    }

    #[setter]
    fn set_merge_equivalent_variables(&mut self, value: bool) -> PyResult<()> {
        self.config.merge_equivalent_variables = value;
        Ok(())
    }

    #[getter]
    fn get_alldifferent_bijection_constraints(&self) -> PyResult<bool> {
        Ok(self.config.alldifferent_bijection_constraints)
    }

    #[setter]
    fn set_alldifferent_bijection_constraints(&mut self, value: bool) -> PyResult<()> {
        self.config.alldifferent_bijection_constraints = value;
        Ok(())
    }

    #[getter]
    fn get_glucose_random_seed(&self) -> PyResult<Option<f64>> {
        Ok(self.config.glucose_random_seed)
    }

    #[setter]
    fn set_glucose_random_seed(&mut self, value: Option<f64>) -> PyResult<()> {
        self.config.glucose_random_seed = value;
        Ok(())
    }

    #[getter]
    fn get_glucose_rnd_init_act(&self) -> PyResult<bool> {
        Ok(self.config.glucose_rnd_init_act)
    }

    #[setter]
    fn set_glucose_rnd_init_act(&mut self, value: bool) -> PyResult<()> {
        self.config.glucose_rnd_init_act = value;
        Ok(())
    }

    #[getter]
    fn get_verbose(&self) -> PyResult<bool> {
        Ok(self.config.verbose)
    }

    #[setter]
    fn set_verbose(&mut self, value: bool) -> PyResult<()> {
        self.config.verbose = value;
        Ok(())
    }

    #[getter]
    fn get_backend(&self) -> PyResult<String> {
        let mode = match self.config.backend {
            Backend::Glucose => "glucose",
            Backend::CaDiCaL => "cadical",
            Backend::External => "external",
        };
        Ok(mode.to_owned())
    }

    #[setter]
    fn set_backend(&mut self, backend: String) -> PyResult<()> {
        if backend == "glucose" {
            self.config.backend = Backend::Glucose;
        } else if backend == "cadical" {
            self.config.backend = Backend::CaDiCaL;
        } else if backend == "external" {
            self.config.backend = Backend::External;
        } else {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "unknown backend: {}",
                backend
            )));
        }
        Ok(())
    }

    #[getter]
    fn get_order_encoding_linear_mode(&self) -> PyResult<String> {
        let mode = match self.config.order_encoding_linear_mode {
            OrderEncodingLinearMode::Cpp => "cpp",
            OrderEncodingLinearMode::Rust => "rust",
            OrderEncodingLinearMode::RustOptimized => "rust-optimized",
        };
        Ok(mode.to_owned())
    }

    #[setter]
    fn set_order_encoding_linear_mode(&mut self, mode: String) -> PyResult<()> {
        if mode == "cpp" {
            self.config.order_encoding_linear_mode = OrderEncodingLinearMode::Cpp;
        } else if mode == "rust" {
            self.config.order_encoding_linear_mode = OrderEncodingLinearMode::Rust;
        } else if mode == "rust-optimized" {
            self.config.order_encoding_linear_mode = OrderEncodingLinearMode::RustOptimized;
        } else {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "unknown linear implementation: {}",
                mode
            )));
        }
        Ok(())
    }

    #[getter]
    fn get_optimize_polarity(&mut self) -> PyResult<bool> {
        Ok(self.config.optimize_polarity)
    }

    #[setter]
    fn set_optimize_polarity(&mut self, value: bool) -> PyResult<()> {
        self.config.optimize_polarity = value;
        Ok(())
    }
}

static mut GLOBAL_CONFIG: PyConfig = PyConfig::new();

#[pyfunction]
fn set_config(config: PyConfig) {
    unsafe {
        GLOBAL_CONFIG = config;
    }
}

#[pyfunction]
fn solver(input: String) -> String {
    let mut bytes = input.as_bytes();
    let (res, _) = csugar_cli(&mut bytes, unsafe { GLOBAL_CONFIG.config.clone() });
    res
}

#[pyfunction]
fn solver_with_perf(input: String) -> (String, HashMap<String, f64>) {
    let mut bytes = input.as_bytes();
    let (res, perf) = csugar_cli(&mut bytes, unsafe { GLOBAL_CONFIG.config.clone() });

    let mut perf_map = HashMap::<String, f64>::new();
    perf_map.insert(String::from("time_normalize"), perf.time_normalize());
    perf_map.insert(String::from("time_encode"), perf.time_encode());
    perf_map.insert(String::from("time_sat_solver"), perf.time_sat_solver());
    perf_map.insert(String::from("decisions"), perf.decisions() as f64);
    perf_map.insert(String::from("propagations"), perf.propagations() as f64);
    perf_map.insert(String::from("conflicts"), perf.conflicts() as f64);

    (res, perf_map)
}

#[pymodule(name = "cspuz_core")]
pub fn cspuz_core_python_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solver, m)?)?;
    m.add_function(wrap_pyfunction!(solver_with_perf, m)?)?;
    m.add_function(wrap_pyfunction!(set_config, m)?)?;
    m.add_class::<PyConfig>()?;

    Ok(())
}
