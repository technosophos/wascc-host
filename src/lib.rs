#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate crossbeam;

pub type Result<T> = std::result::Result<T, errors::Error>;
pub use actor::Actor;
pub use capability::Capability;
pub use middleware::Middleware;
pub use wapc::prelude::WasiParams;

mod actor;
mod authz;
mod capability;
pub mod dispatch;
pub mod errors;
pub mod host;
mod middleware;
pub mod plugins;
mod router;
