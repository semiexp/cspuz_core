#[cfg(feature = "backend-external")]
pub mod external;

#[cfg(feature = "backend-cadical")]
pub mod cadical;

#[cfg(feature = "backend-glucose-rs")]
#[path = "glucose_rs.rs"]
pub mod glucose;

#[cfg(not(feature = "backend-glucose-rs"))]
pub mod glucose;
