#[macro_use]
mod error;

mod address;
mod index;
mod object;
mod query;
mod query_runner;
mod schema;
mod term;

mod native;

pub type Result<T> = std::result::Result<T, error::BurkazError>;
